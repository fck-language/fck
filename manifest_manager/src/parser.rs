use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
pub struct ManifestFile {
    package: Package,
    dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Package {
    name: String,
    version: [u8; 3],
    authors: Vec<String>,
    edition: [u8; 3],
    flavour: String,
}

#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: DependencyVersion,
}

#[derive(Debug)]
pub enum DependencyVersion {
    Literal([u8; 3]),
    Minimum([u8; 3]),
    MostRecent,
}

impl Display for DependencyVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DependencyVersion::MostRecent => "* (most recent)".to_string(),
            DependencyVersion::Literal(v) => v[1..].iter().fold(format!("{}", v[0]), |acc, arg| format!("{}.{}", acc, arg)),
            DependencyVersion::Minimum(v) => {
                let mut out = v[1..].iter().fold(format!("{}", v[0]), |acc, arg| format!("{}.{}", acc, arg));
                out.push_str("^ (at least)");
                out
            }
        })
    }
}

impl ManifestFile {
    pub fn new() -> ManifestFile {
        ManifestFile {
            package: Package {
                name: "".to_string(),
                version: [0u8, 0u8, 0u8],
                authors: vec![],
                edition: [0u8, 0u8, 0u8],
                flavour: "".to_string(),
            },
            dependencies: vec![],
        }
    }

    pub fn parse(&mut self, path_to_file: &str) -> &mut ManifestFile {
        let file_contents = read_to_string(path_to_file).unwrap();
        let mut file = file_contents.split("\n").collect::<Vec<&str>>();
        let mut current_mode = "";
        let match_re = Regex::new(r"([^\s]+)\s*=\s*([^\r\n]+)").unwrap();
        let mut captures: regex::Captures;
        for i in file {
            if i.is_empty() {
                continue;
            }
            if i.chars().nth(0).unwrap() == '[' {
                current_mode = i;
                continue;
            }
            captures = match match_re.captures_iter(i).nth(0) {
                Some(m) => m,
                None => continue
            };
            match current_mode {
                "[package]" => {
                    match &captures[1] {
                        "name" => self.package.name = captures[2].to_string(),
                        "version" => {
                            let version_match = Regex::new(r"(\d+).(\d+).(\d+)").unwrap().captures_iter(&captures[2]).nth(0).unwrap();
                            self.package.version = [*&version_match[1].parse::<u8>().unwrap(), *&version_match[2].parse::<u8>().unwrap(), *&version_match[3].parse::<u8>().unwrap()]
                        }
                        "authors" => {
                            for cap in Regex::new("['\"]([^'\"]*)['\"]").unwrap().captures_iter(&captures[2]) {
                                self.package.authors.push(cap.get(1).unwrap().as_str().to_string());
                            }
                        }
                        "edition" => {
                            let version_match = Regex::new(r"(\d+).(\d+).(\d+)").unwrap().captures_iter(&captures[2]).nth(0).unwrap();
                            self.package.edition = [*&version_match[1].parse::<u8>().unwrap(), *&version_match[2].parse::<u8>().unwrap(), *&version_match[3].parse::<u8>().unwrap()]
                        }
                        "flavour" => self.package.flavour = captures[2].to_string(),
                        _ => {}
                    }
                }
                "[dependencies]" => {
                    let version = if &captures[2] == "*" {
                        DependencyVersion::MostRecent
                    } else {
                        let v = Regex::new(r"(\d+).(\d+).(\d+)").unwrap().captures_iter(&captures[2]).nth(0).unwrap();
                        let ver = [*&v[1].parse::<u8>().unwrap(), *&v[2].parse::<u8>().unwrap(), *&v[3].parse::<u8>().unwrap()];
                        if &captures[2].chars().nth(captures[2].len() - 1).unwrap() == &'^' {
                            DependencyVersion::Minimum(ver)
                        } else {
                            DependencyVersion::Literal(ver)
                        }
                    };
                    self.dependencies.push(Dependency { name: captures[1].to_string(), version })
                }
                _ => {}
            }
        }
        self
    }
}

impl Display for ManifestFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn version_display(v: &[u8; 3]) -> String {
            v[1..].iter().fold(format!("{}", v[0]), |acc, arg| format!("{}.{}", acc, arg))
        }
        let authors = if self.package.authors.len() == 0 {
            "None?".to_string()
        } else {
            self.package.authors[1..].iter().fold(format!("{}", self.package.authors[0]), |acc, arg| format!("{}, {}", acc, arg))
        };
        write!(f, "Package:{}\nfck version info:{}\nDependencies:{}", vec![["name", &self.package.name],
                                                                           ["version", &*format!("{}", version_display(&self.package.version))],
                                                                           ["authors", &*authors],
        ].iter().fold(String::new(), |acc, arg| format!("{}\n{:>17} = {}", acc, arg[0], arg[1])),
               format!("\n{:>17} = {}\n{:>17} = {}", "version", version_display(&self.package.edition), "flavour", &self.package.flavour),
               self.dependencies.iter().fold(String::new(), |acc, arg| format!("{}\n{:>17} = {}", acc, arg.name, arg.version)))
    }
}

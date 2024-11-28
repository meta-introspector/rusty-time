//use org::parser::Parser;
//use orgize::Org;
use orgize::rowan::ast::AstNode;
use tracing_subscriber::fmt::format::FmtSpan;

//..fn main() {
//    let args: Vec<_> = args().collect();

use orgize::{
    export::{from_fn, Container, Event},
    SyntaxElement,
    SyntaxKind,
    Org,
//    ast::{LIST_ITEM_CONTENT,PARAGRAPH,TEXT},
    //syntax::{OrgLanguage, SyntaxNode},
};

use git2::{Config, Repository, Submodule};
use std::{
    fs,
    path::{Path, PathBuf},
};


fn main() {

    tracing_subscriber::fmt()
        .without_time()
        .with_file(true)
        .with_span_events(FmtSpan::NEW)
        .with_line_number(true)
//        .with_max_level(tracing::Level::TRACE)
        .with_file(false)
        .with_line_number(false)
        .init();

    let args: Vec<String> = std::env::args().collect();    
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename.org>");
        return;
    }
    let file_path = Path::new(&args[1]);
    //println!("got this path: {}", file_path.display());   
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let parse_org = Org::parse(&contents);
    let mut parser_output = parse_org;    
    let mut handler = from_fn(|event| {
	match event {
	    Event::Enter(Container::Headline(headline)) => {
		//println!("headline: {:#?}", headline);
	    }
	    Event::Enter(Container::Section(section)) => {
	        //println!("section syntax other: {:#?}", orgize::ast::Section::syntax(&section).first_child().expect("oops"));
		
		for c in orgize::ast::Section::syntax(&section).first_child().expect("oops").children() {
	      	    //println!("test: {:#?}", c);
		    for c2 in c.children() {
			//println!("test2: {:#?}", c2);
			for c3 in c2.children() {
			    println!("test3: {:#?}", c3.text().to_string().trim());
			    //for c4 in c3.children() {
			    //		println!("test4: {:#?}", c4);
			    //}
			}
		    }		
		}
	    }
	    _=> {
		//println!("OTTHER: {:#?}", event);
	    }
	}
    });
    parser_output.traverse(&mut handler);
    
    
//    parser_output.
//    let parser = Parser::default();
//    let document = parser.parse(&content).unwrap();

//    let git_repo = Repository::open(".").expect("Failed to open repository");
//    let submodules_path = ".gitmodules";

    // Ensure .gitmodules exists
//    if !Path::new(submodules_path).exists() {
//        fs::write(submodules_path, "").expect("Failed to create .gitmodules file");
//    }

    // for item in document.items() {
    //     if let org::Item::ListItem { content, .. } = item {
    //         if let Some(url) = parse_url(content) {
    //             check_and_add_submodule(&mut git_repo, &url);
    //         }
    //     }
    // }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = std::env::args().collect();
    
//     if args.len() < 2 {
//         eprintln!("Usage: cargo run <filename.org>");
//         return Ok(());
//     }

//     let file_path = Path::new(&args[1]);
//     println!("got this path: {}", file_path.display());   

//     let contents = fs::read_to_string(file_path)?;
    
//     // Parse the Org-mode content
//     let parse_org = Org::Parser::new().parse_all(&contents);
//     let mut parser_output = parse_org.unwrap();

//     println!("got this item: {:#?}", parser_output);

//     // Extract URLs from the parsed list


//     // // Path to the .gitmodules file
//     // let gitmodules_path = repo_path.join(".gitmodules");
    
//     // // Read the current submodules
//     // let mut existing_submodules: Vec<String> = Vec::new();
//     // if gitmodules_path.exists() {
//     //     let gitmodules_content = fs::read_to_string(&gitmodules_path)?;
//     //     for line in gitmodules_content.lines() {
//     //         if line.starts_with("path=") {
//     //             if let Some(path_start) = line.find('=') {
//     //                 existing_submodules.push(line[path_start + 1..].trim().to_string());
//     //             }
//     //         }
//     //     }
//     // }

//     // // Add or symlink new submodules
//     // for url in urls {
//     //     if !existing_submodules.contains(&url) {
//     //         add_or_symlink_submodule(&repo_path, &url)?;
//     //     } else {
//     //         println!("Submodule already exists: {}", url);
//     //     }
//     // }

//     Ok(())
// }

// fn extract_urls(org_node: &Org::SyntaxNode) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let mut urls = Vec::new();

//     fn traverse(node: &Org::SyntaxNode, parent_indent: usize, f: &mut dyn FnMut(&Org::SyntaxNode)) {
//         if node.content.indent() > parent_indent {
//             f(node);
//         }
//         for child in &node.children {
//             traverse(child, node.content.indent(), f);
//         }
//     }

//     let mut handler = |node: &Org::SyntaxNode| {
//         match &node.content.value {
//             Org::Value::String(s) => {
//                 if s.contains("https://github.com") && s.ends_with(".git") {
//                     urls.push(s.to_string());
//                 }
//             },
//             _ => {}
//         }
//     };

//     traverse(org_node, 0, &mut handler);

//     Ok(urls)
// }

// fn add_or_symlink_submodule(repo_path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let repo = Repository::open(repo_path)?;
//     let config = repo.config()?;
    
//     // Extract the path from the URL
//     let path_start = url.find("/repos/").unwrap_or_else(|| url.find("github.com/").unwrap()) + 7;
//     let path_end = url.rfind('/').unwrap();
//     let module_name = &url[path_start..path_end];
//     let module_path = repo_path.join(module_name);
    
//     // Clone the submodule if it doesn't exist
//     if !module_path.exists() {
//         git2::build::RepoBuilder::new()
//             .clone(url, &repo_path)?;
        
//         println!("Cloned submodule: {}", url);
//     } else {
//         println!("Symlinking submodule: {}", url);
        
//         // Symlink the existing directory to the submodule path
//         std::os::unix::fs::symlink(&module_path, module_path.join(".git"))?;
//     }
    
//     // Update .gitmodules
//     let mut f = fs::File::options()
//         .append(true)
//         .create_if_missing(true)
//         .open(repo_path.join(".gitmodules"))?;
    
//     writeln!(f, "[submodule \"{}\"]\n  path = {}\n  url = {}", module_name, module_name, url)?;

//     Ok(())
// }

// fn repo_path() -> PathBuf {
//     let current_dir = std::env::current_dir().unwrap();
//     current_dir.join(".git")
// }

// fn parse_url(content: &str) -> Option<String> {
//     let start_index = content.find("https://github.com/")?;
//     let end_index = content[start_index..].find('\n')?;
//     Some(format!("{}{}", &content[start_index..start_index + end_index], content[end_index..].trim()))
// }

// fn check_and_add_submodule(repo: &mut Repository, url: &str) {
//     if is_submodule_exists(repo, url) {
//         println!("Submodule already exists for {}", url);
//     } else {
//         add_submodule(repo, url).expect("Failed to add submodule");
//     }
// }

// fn is_submodule_exists(repo: &Repository, url: &str) -> bool {
//     let config = repo.config().unwrap();
//     //if let Some(entries) = config.get_value_multi_str("submodule") {
// //        entries.iter().any(|entry| entry == url)
//   //  } else {
// //        false
//     //   }
//     return false;
// }
// // git2::Result<()>
// //type Result {};
// fn add_submodule(repo: &mut Repository, url: &str) -> Result<(), git2::Error> {
// //     let path = PathBuf::from(url.split('/').last().unwrap_or("default").replace('-', "_"));
// //     repo.submodule_add_path(&path, url).expect("Failed to add submodule path");
// //     let submodule = Submodule::init(repo, &url, &path)?;
// //     // 69 |     submodule.update(false)?;
// //     //|               ^^^^^^ method not found in `()`
// //     //submodule.update(false)?;
//      Ok(())
//  }


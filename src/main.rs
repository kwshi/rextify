use structopt::StructOpt;

#[derive(structopt::StructOpt, Debug)]
struct Cmd {
    path: std::path::PathBuf,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), Error> {
    let cmd = Cmd::from_args();
    let contents = std::fs::read_to_string(cmd.path)?;
    let source = linttex::source::Source::new(&contents);
    let mut lexer = linttex::lexer::Lexer::new(&source);
    //loop {
    //    match lexer.next() {
    //        Ok(token) => {}
    //        Err(err) => {
    //            source.pprint_loc(err.loc());
    //            println!("{:?}", err);
    //            break;
    //        }
    //    }
    //}

    let parser = linttex::parser::Parser::new(lexer);
    match parser.parse() {
        Ok(tree) => println!("{:#?}", tree),
        Err(err) => println!("{:#?}", err),
    }

    println!("{}", std::mem::size_of::<linttex::syntax::ast::Root>());

    //loop {
    //    match lexer.next() {
    //        Ok(token) => println!("{:?}", token),
    //        Err(err) => {
    //            source.pprint_loc(err.loc());
    //            println!("{:?}", err);
    //            break;
    //        }
    //    }
    //}

    Ok(())
}

var searchIndex = JSON.parse('{\
"cargo_servum":{"doc":"","i":[[5,"main","cargo_servum","",null,[[]]]],"p":[]},\
"servum":{"doc":"Servum is a simple static server. Not more. Not less.","i":[[0,"cli","servum","CLI arguments parser and help",null,null],[0,"config","servum::cli","",null,null],[3,"Config","servum::cli::config","Rudimentary argument parsing and user configuration.",null,null],[12,"address","","",0,null],[12,"base_dir","","",0,null],[12,"list_dir","","",0,null],[12,"port","","",0,null],[12,"threads","","",0,null],[12,"verbose","","",0,null],[11,"new","","Create a new user configuration from environment …",0,[[],["config",3]]],[11,"parse_args","","Parse environment arguments and update a user [<code>Config</code>] …",0,[[["config",3]],[["clierror",4],["result",4]]]],[11,"help_header","","Return the help menu header common to all help menus.",0,[[]]],[11,"help_long","","Return the help menu in its verbose form.",0,[[],["string",3]]],[11,"help_short","","Return the help menu in its short form.",0,[[],["string",3]]],[0,"err","servum::cli","",null,null],[4,"CliError","servum::cli::err","Possible errors encountered when parsing user arguments.",null,null],[13,"InvalidArg","","",1,null],[13,"InvalidVal","","",1,null],[13,"MissingVal","","",1,null],[13,"IOError","","",1,null],[0,"tui","servum::cli","",null,null],[5,"print_logo","servum::cli::tui","Print ASCII Art of <code>servum</code> to the console.",null,[[]]],[5,"print_info","","Print general information about help and quitting to the …",null,[[]]],[5,"print_config","","Print information about the current user [<code>Config</code>] to the …",null,[[["arc",3],["config",3]]]],[5,"print_verbose_header","","Print table header of verbose output to the console.",null,[[]]],[5,"print_verbose_stats","","Print verbose stats about a request to the console.",null,[[["instant",3],["httpresponse",3],["httprequest",3]]]],[3,"Config","servum::cli","Rudimentary argument parsing and user configuration.",null,null],[12,"address","","",0,null],[12,"base_dir","","",0,null],[12,"list_dir","","",0,null],[12,"port","","",0,null],[12,"threads","","",0,null],[12,"verbose","","",0,null],[0,"files","servum","Filesystem and path utilities",null,null],[0,"file","servum::files","",null,null],[3,"File","servum::files::file","Wrapper struct around [<code>std::fs::DirEntry</code>]",null,null],[12,"0","","",2,null],[0,"mime","servum::files","",null,null],[5,"guess_mime_type","servum::files::mime","Guess a file\'s MIME type based on it\'s extension",null,[[["path",3]],["option",4]]],[0,"path","servum::files","",null,null],[5,"from_hex","servum::files::path","Try decoding hex encoding after <code>%</code> in URIs.",null,[[["iter",3]],["option",4]]],[5,"decode_percents","","Decode percent-encoded URIs",null,[[],["pathbuf",3]]],[5,"normalize_path","","Normalize a file path",null,[[["path",3]],["pathbuf",3]]],[5,"process_path","","Process a file path",null,[[["path",3]],["pathbuf",3]]],[0,"http","servum","HTTP utilities",null,null],[0,"handler","servum::http","",null,null],[5,"list_dir","servum::http::handler","List a directory for a given <code>Path</code>.",null,[[["path",3]],[["vec",3],["result",6]]]],[5,"handle_connection","","Handle incoming HTTP requests.",null,[[["arc",3],["config",3],["httprequest",3]],["httpresponse",3]]],[0,"html","servum::http","",null,null],[5,"html_doc","servum::http::html","Generate an HTML document containing title, lead and …",null,[[],["string",3]]],[0,"request","servum::http","",null,null],[3,"HTTPRequest","servum::http::request","A struct used to represent an HTTP request.",null,null],[12,"method","","",3,null],[12,"filepath","","",3,null],[11,"new","","Create a new HTTPRequest from a [<code>std::net::TcpStream</code>] …",3,[[],[["result",4],["httprequesterror",4]]]],[0,"request_err","servum::http","",null,null],[4,"HTTPRequestError","servum::http::request_err","HTTPRequestError representing possible HTTP request …",null,null],[13,"NoMethod","","",4,null],[13,"NoPath","","",4,null],[13,"Utf8Error","","",4,null],[0,"response","servum::http","",null,null],[3,"HTTPResponse","servum::http::response","A struct representing an HTTP response.",null,null],[12,"status","","",5,null],[12,"mime","","",5,null],[12,"body","","",5,null],[11,"new","","Create a new HTTPResponse from an [<code>HTTPStatus</code>], an …",5,[[["vec",3],["httpstatus",3],["option",4],["result",6]]]],[11,"header","","Generate a HTTP header by from the response and return it …",5,[[],["vec",3]]],[11,"into_bytes","","Turn the HTTPResponse into a vector of bytes by consuming …",5,[[],["vec",3]]],[0,"status","servum::http","",null,null],[3,"HTTPStatus","servum::http::status","A simple struct representing HTTP status responses.",null,null],[12,"code","","",6,null],[12,"msg","","",6,null],[12,"comment","","",6,null],[11,"new","","Create a new instance of HTTPStatus with the given code, …",6,[[["string",3],["option",4]]]],[11,"to_html","","Create a HTML document from the status instance by …",6,[[],["string",3]]],[5,"handle_connection","servum::http","Handle incoming HTTP requests.",null,[[["arc",3],["config",3],["httprequest",3]],["httpresponse",3]]],[5,"html_doc","","Generate an HTML document containing title, lead and …",null,[[],["string",3]]],[3,"HTTPRequest","","A struct used to represent an HTTP request.",null,null],[12,"method","","",3,null],[12,"filepath","","",3,null],[4,"HTTPRequestError","","HTTPRequestError representing possible HTTP request …",null,null],[13,"NoMethod","","",4,null],[13,"NoPath","","",4,null],[13,"Utf8Error","","",4,null],[3,"HTTPResponse","","A struct representing an HTTP response.",null,null],[12,"status","","",5,null],[12,"mime","","",5,null],[12,"body","","",5,null],[3,"HTTPStatus","","A simple struct representing HTTP status responses.",null,null],[12,"code","","",6,null],[12,"msg","","",6,null],[12,"comment","","",6,null],[0,"multiprocessing","servum","ThreadPool implementation from the Rust Book",null,null],[0,"message","servum::multiprocessing","",null,null],[6,"Job","servum::multiprocessing::message","",null,null],[4,"Message","","Message to a <code>threadpool::Worker</code>",null,null],[13,"NewJob","","",7,null],[13,"Terminate","","",7,null],[0,"threadpool","servum::multiprocessing","",null,null],[3,"ThreadPool","servum::multiprocessing::threadpool","ThreadPool for multi-thread computations.",null,null],[12,"workers","","",8,null],[12,"sender","","",8,null],[11,"new","","Create a new ThreadPool.",8,[[],["threadpool",3]]],[11,"execute","","Execute a job closure",8,[[]]],[0,"worker","servum::multiprocessing","",null,null],[3,"Worker","servum::multiprocessing::worker","ThreadPool Worker",null,null],[12,"id","","",9,null],[12,"thread","","",9,null],[11,"new","","Create a new Worker",9,[[["arc",3],["mutex",3]],["worker",3]]],[4,"Message","servum::multiprocessing","Message to a <code>threadpool::Worker</code>",null,null],[13,"NewJob","","",7,null],[13,"Terminate","","",7,null],[3,"ThreadPool","","ThreadPool for multi-thread computations.",null,null],[12,"workers","","",8,null],[12,"sender","","",8,null],[3,"Worker","","ThreadPool Worker",null,null],[12,"id","","",9,null],[12,"thread","","",9,null],[11,"from","servum::cli::config","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","servum::cli::err","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","servum::files::file","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_string","","",2,[[],["string",3]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","servum::http::request","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","servum::http::request_err","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_string","","",4,[[],["string",3]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","servum::http::response","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_string","","",5,[[],["string",3]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","servum::http::status","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","servum::multiprocessing::message","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","servum::multiprocessing::threadpool","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","servum::multiprocessing::worker","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"drop","servum::multiprocessing::threadpool","",8,[[]]],[11,"from","servum::cli::err","",1,[[["error",3]],["clierror",4]]],[11,"from","servum::http::request_err","",4,[[["utf8error",3]],["httprequesterror",4]]],[11,"from","servum::http::response","",5,[[["error",3]]]],[11,"from","","",5,[[["httpstatus",3]]]],[11,"from","servum::http::status","",6,[[]]],[11,"from","","",6,[[["result",6]]]],[11,"from","","",6,[[["error",3]]]],[11,"default","servum::cli::config","",0,[[]]],[11,"fmt","servum::cli::err","",1,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::request","",3,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::request_err","",4,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::response","",5,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::status","",6,[[["formatter",3]],["result",6]]],[11,"fmt","servum::cli::err","",1,[[["formatter",3]],["result",6]]],[11,"fmt","servum::files::file","",2,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::request","",3,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::request_err","",4,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::response","",5,[[["formatter",3]],["result",6]]],[11,"fmt","servum::http::status","",6,[[["formatter",3]],["result",6]]],[11,"source","servum::cli::err","",1,[[],[["error",8],["option",4]]]],[11,"source","servum::http::request_err","",4,[[],[["error",8],["option",4]]]]],"p":[[3,"Config"],[4,"CliError"],[3,"File"],[3,"HTTPRequest"],[4,"HTTPRequestError"],[3,"HTTPResponse"],[3,"HTTPStatus"],[4,"Message"],[3,"ThreadPool"],[3,"Worker"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);
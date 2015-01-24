var searchIndex = {};
searchIndex['markgen'] = {"items":[],"paths":[]};
searchIndex['markov'] = {"items":[[0,"","markov","A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type. This\nuses HashMaps internally, and so Eq and Hash are both required."],[3,"Chain","","A generic [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) for almost any type. This\nuses HashMaps internally, and so Eq and Hash are both required."],[3,"SizedChainIterator","","A sized iterator over a Markov chain."],[3,"InfiniteChainIterator","","An infinite iterator over a Markov chain."],[6,"SizedChainStringIterator","","A sized iterator over a Markov chain of strings."],[6,"InfiniteChainStringIterator","","An infinite iterator over a Markov chain of strings."],[8,"Chainable","","The definition of all types that can be used in a Chain."],[11,"fmt","","",0],[11,"eq","","",0],[11,"ne","","",0],[11,"decode","","",0],[11,"encode","","",0],[11,"new","","Constructs a new Markov chain using the given tokens as the marked starting and ending\npoints for generation.",0],[11,"is_empty","","Determines whether or not the chain is empty. A chain is considered empty if nothing has\nbeen fed into it.",0],[11,"feed","","Feeds the chain a collection of tokens. This operation is O(n) where n is the number of\ntokens to be fed into the chain.",0],[11,"generate","","Generates a collection of tokens from the chain. This operation is O(mn) where m is the\nlength of the generated collection, and n is the number of possible states from a given\nstate.",0],[11,"generate_from_token","","Generates a collection of tokens from the chain, starting with the given token. This\noperation is O(mn) where m is the length of the generated collection, and n is the number\nof possible states from a given state. This returns an empty vector if the token is not\nfound.",0],[11,"iter","","Produces an infinite iterator of generated token collections.",0],[11,"iter_for","","Produces an iterator for the specified number of generated token collections.",0],[11,"load","","Loads a chain from a JSON file at the specified path.",0],[11,"load_utf8","","Loads a chain from a JSON file using a string path.",0],[11,"save","","Saves a chain to a JSON file at the specified path.",0],[11,"save_utf8","","Saves a chain to a JSON file using a string path.",0],[11,"for_strings","","Creates a new Chain intended specifically for strings. This uses the Unicode start of text\nand end of text control characters as the starting and ending tokens for the chain.",0],[11,"feed_str","","Feeds a string of text into the chain.",0],[11,"feed_file","","Feeds a properly formatted file into the chain. This file should be formatted such that\neach line is a new sentence. Punctuation may be included if it is desired.",0],[11,"generate_str","","Generates a random string of text.",0],[11,"generate_str_from_token","","Generates a random string of text starting with the desired token. This returns an empty\nstring if the token is not found.",0],[11,"str_iter","","Produces an infinite iterator of generated strings.",0],[11,"str_iter_for","","Produces a sized iterator of generated strings.",0],[6,"Item","",""],[11,"next","","",1],[11,"size_hint","","",1],[6,"Item","",""],[11,"next","","",2]],"paths":[[3,"Chain"],[3,"SizedChainIterator"],[3,"InfiniteChainIterator"]]};

searchIndex['rustc-serialize'] = {"items":[[0,"","rustc-serialize","Support code for encoding and decoding types."],[11,"encode","collections::string","",0],[11,"decode","","",0],[11,"encode","alloc::boxed","",1],[11,"decode","","",1],[11,"decode","","",1],[11,"encode","alloc::rc","",2],[11,"decode","","",2],[11,"encode","collections::vec","",3],[11,"decode","","",3],[11,"encode","core::option","",4],[11,"decode","","",4],[11,"encode","std::path::posix","",5],[11,"decode","","",5],[11,"encode","std::path::windows","",6],[11,"decode","","",6],[11,"encode","core::cell","",7],[11,"decode","","",7],[11,"encode","","",8],[11,"decode","","",8],[11,"encode","alloc::arc","",9],[11,"decode","","",9],[11,"encode","collections::dlist","",10],[11,"decode","","",10],[11,"encode","collections::ring_buf","",11],[11,"decode","","",11],[11,"encode","collections::btree::map","",12],[11,"decode","","",12],[11,"encode","collections::btree::set","",13],[11,"decode","","",13],[11,"encode","std::collections::hash::map","",14],[11,"decode","","",14],[11,"encode","std::collections::hash::set","",15],[11,"decode","","",15],[11,"encode","collections::vec_map","",16],[11,"decode","","",16],[0,"base64","rustc-serialize","Base64 binary-to-text encoding"],[3,"Config","rustc-serialize::base64","Contains configuration parameters for `to_base64`."],[12,"char_set","","Character set to use",17],[12,"newline","","Newline to use",17],[12,"pad","","True to pad output with `=` characters",17],[12,"line_length","","`Some(len)` to wrap lines at `len`, `None` to disable line wrapping",17],[4,"CharacterSet","","Available encoding character sets"],[13,"Standard","","The standard character set (uses `+` and `/`)",18],[13,"UrlSafe","","The URL safe character set (uses `-` and `_`)",18],[4,"Newline","","Available newline types"],[13,"LF","","A linefeed (i.e. Unix-style newline)",19],[13,"CRLF","","A carriage return and a linefeed (i.e. Windows-style newline)",19],[4,"FromBase64Error","","Errors that can occur when decoding a base64 encoded string"],[13,"InvalidBase64Byte","","The input contained a character not part of the base64 format",20],[13,"InvalidBase64Length","","The input had an invalid length",20],[7,"STANDARD","","Configuration for RFC 4648 standard base64 encoding"],[7,"URL_SAFE","","Configuration for RFC 4648 base64url encoding"],[7,"MIME","","Configuration for RFC 2045 MIME base64 encoding"],[8,"ToBase64","","A trait for converting a value to base64 encoding."],[10,"to_base64","","Converts the value of `self` to a base64 value following the specified\nformat configuration, returning the owned string.",21],[8,"FromBase64","","A trait for converting from base64 encoded values."],[10,"from_base64","","Converts the value of `self`, interpreted as base64 encoded data, into\nan owned vector of bytes, returning the vector.",22],[11,"fmt","","",20],[11,"description","","",20],[11,"fmt","","",20],[0,"hex","rustc-serialize","Hex binary-to-text encoding"],[4,"FromHexError","rustc-serialize::hex","Errors that can occur when decoding a hex encoded string"],[13,"InvalidHexCharacter","","The input contained a character not part of the hex format",23],[13,"InvalidHexLength","","The input had an invalid length",23],[8,"ToHex","","A trait for converting a value to hexadecimal encoding"],[10,"to_hex","","Converts the value of `self` to a hex value, returning the owned\nstring.",24],[8,"FromHex","","A trait for converting hexadecimal encoded values"],[10,"from_hex","","Converts the value of `self`, interpreted as hexadecimal encoded data,\ninto an owned vector of bytes, returning the vector.",25],[11,"fmt","","",23],[11,"description","","",23],[11,"fmt","","",23],[0,"json","rustc-serialize","JSON parsing and serialization"],[3,"PrettyJson","rustc-serialize::json",""],[3,"AsJson","",""],[3,"AsPrettyJson","",""],[3,"Encoder","","A structure for implementing serialization to JSON."],[3,"PrettyEncoder","","Another encoder for JSON, but prints out human-readable JSON instead of\ncompact data"],[3,"Stack","","A Stack represents the current position of the parser in the logical\nstructure of the JSON stream.\nFor example foo.bar[3].x"],[3,"Parser","","A streaming JSON parser implemented as an iterator of JsonEvent, consuming\nan iterator of char."],[3,"Builder","","A Builder consumes a json::Parser to create a generic Json structure."],[3,"Decoder","","A structure to decode JSON to values in rust."],[4,"Json","","Represents a json value"],[13,"I64","","",26],[13,"U64","","",26],[13,"F64","","",26],[13,"String","","",26],[13,"Boolean","","",26],[13,"Array","","",26],[13,"Object","","",26],[13,"Null","","",26],[4,"ErrorCode","","The errors that can arise while parsing a JSON stream."],[13,"InvalidSyntax","","",27],[13,"InvalidNumber","","",27],[13,"EOFWhileParsingObject","","",27],[13,"EOFWhileParsingArray","","",27],[13,"EOFWhileParsingValue","","",27],[13,"EOFWhileParsingString","","",27],[13,"KeyMustBeAString","","",27],[13,"ExpectedColon","","",27],[13,"TrailingCharacters","","",27],[13,"TrailingComma","","",27],[13,"InvalidEscape","","",27],[13,"InvalidUnicodeCodePoint","","",27],[13,"LoneLeadingSurrogateInHexEscape","","",27],[13,"UnexpectedEndOfHexEscape","","",27],[13,"UnrecognizedHex","","",27],[13,"NotFourDigit","","",27],[13,"NotUtf8","","",27],[4,"ParserError","",""],[13,"SyntaxError","","msg, line, col",28],[13,"IoError","","",28],[4,"DecoderError","",""],[13,"ParseError","","",29],[13,"ExpectedError","","",29],[13,"MissingFieldError","","",29],[13,"UnknownVariantError","","",29],[13,"ApplicationError","","",29],[4,"EncoderError","",""],[13,"FmtError","","",30],[13,"BadHashmapKey","","",30],[4,"JsonEvent","","The output of the streaming parser."],[13,"ObjectStart","","",31],[13,"ObjectEnd","","",31],[13,"ArrayStart","","",31],[13,"ArrayEnd","","",31],[13,"BooleanValue","","",31],[13,"I64Value","","",31],[13,"U64Value","","",31],[13,"F64Value","","",31],[13,"StringValue","","",31],[13,"NullValue","","",31],[13,"Error","","",31],[4,"StackElement","","StackElements compose a Stack.\nFor example, Key(\"foo\"), Key(\"bar\"), Index(3) and Key(\"x\") are the\nStackElements compositing the stack that represents foo.bar[3].x"],[13,"Index","","",32],[13,"Key","","",32],[5,"error_str","","Returns a readable error string for a given error code."],[5,"decode","","Shortcut function to decode a JSON `&str` into an object"],[5,"encode","","Shortcut function to encode a `T` into a JSON `String`"],[5,"as_json","","Create an `AsJson` wrapper which can be used to print a value as JSON\non-the-fly via `write!`"],[5,"as_pretty_json","","Create an `AsPrettyJson` wrapper which can be used to print a value as JSON\non-the-fly via `write!`"],[6,"Array","",""],[6,"Object","",""],[6,"BuilderError","",""],[6,"EncodeResult","",""],[6,"DecodeResult","",""],[8,"ToJson","","A trait for converting values to JSON"],[10,"to_json","","Converts the value of `self` to an instance of JSON",33],[11,"fmt","","",26],[11,"partial_cmp","","",26],[11,"lt","","",26],[11,"le","","",26],[11,"gt","","",26],[11,"ge","","",26],[11,"eq","","",26],[11,"ne","","",26],[11,"clone","","",26],[11,"eq","","",27],[11,"ne","","",27],[11,"clone","","",27],[11,"fmt","","",28],[11,"eq","","",28],[11,"ne","","",28],[11,"clone","","",28],[11,"fmt","","",29],[11,"eq","","",29],[11,"ne","","",29],[11,"clone","","",29],[11,"fmt","","",30],[11,"fmt","","",27],[11,"description","","",29],[11,"cause","","",29],[11,"fmt","","",29],[11,"description","","",28],[11,"fmt","","",28],[11,"description","","",30],[11,"fmt","","",30],[11,"from_error","","",30],[11,"new","","Creates a new JSON encoder whose output will be written to the writer\nspecified.",34],[6,"Error","",""],[11,"emit_nil","","",34],[11,"emit_usize","","",34],[11,"emit_u64","","",34],[11,"emit_u32","","",34],[11,"emit_u16","","",34],[11,"emit_u8","","",34],[11,"emit_isize","","",34],[11,"emit_i64","","",34],[11,"emit_i32","","",34],[11,"emit_i16","","",34],[11,"emit_i8","","",34],[11,"emit_bool","","",34],[11,"emit_f64","","",34],[11,"emit_f32","","",34],[11,"emit_char","","",34],[11,"emit_str","","",34],[11,"emit_enum","","",34],[11,"emit_enum_variant","","",34],[11,"emit_enum_variant_arg","","",34],[11,"emit_enum_struct_variant","","",34],[11,"emit_enum_struct_variant_field","","",34],[11,"emit_struct","","",34],[11,"emit_struct_field","","",34],[11,"emit_tuple","","",34],[11,"emit_tuple_arg","","",34],[11,"emit_tuple_struct","","",34],[11,"emit_tuple_struct_arg","","",34],[11,"emit_option","","",34],[11,"emit_option_none","","",34],[11,"emit_option_some","","",34],[11,"emit_seq","","",34],[11,"emit_seq_elt","","",34],[11,"emit_map","","",34],[11,"emit_map_elt_key","","",34],[11,"emit_map_elt_val","","",34],[11,"new","","Creates a new encoder whose output will be written to the specified writer",35],[11,"set_indent","","Set the number of spaces to indent for each level.\nThis is safe to set during encoding.",35],[6,"Error","",""],[11,"emit_nil","","",35],[11,"emit_usize","","",35],[11,"emit_u64","","",35],[11,"emit_u32","","",35],[11,"emit_u16","","",35],[11,"emit_u8","","",35],[11,"emit_isize","","",35],[11,"emit_i64","","",35],[11,"emit_i32","","",35],[11,"emit_i16","","",35],[11,"emit_i8","","",35],[11,"emit_bool","","",35],[11,"emit_f64","","",35],[11,"emit_f32","","",35],[11,"emit_char","","",35],[11,"emit_str","","",35],[11,"emit_enum","","",35],[11,"emit_enum_variant","","",35],[11,"emit_enum_variant_arg","","",35],[11,"emit_enum_struct_variant","","",35],[11,"emit_enum_struct_variant_field","","",35],[11,"emit_struct","","",35],[11,"emit_struct_field","","",35],[11,"emit_tuple","","",35],[11,"emit_tuple_arg","","",35],[11,"emit_tuple_struct","","",35],[11,"emit_tuple_struct_arg","","",35],[11,"emit_option","","",35],[11,"emit_option_none","","",35],[11,"emit_option_some","","",35],[11,"emit_seq","","",35],[11,"emit_seq_elt","","",35],[11,"emit_map","","",35],[11,"emit_map_elt_key","","",35],[11,"emit_map_elt_val","","",35],[11,"encode","","",26],[11,"from_reader","","Decodes a json value from an `&mut io::Reader`",26],[11,"from_str","","Decodes a json value from a string",26],[11,"pretty","","Borrow this json object as a pretty object to generate a pretty\nrepresentation for it via `Show`.",26],[11,"find","","If the Json value is an Object, returns the value associated with the provided key.\nOtherwise, returns None.",26],[11,"find_path","","Attempts to get a nested Json Object for each key in `keys`.\nIf any key is found not to exist, find_path will return None.\nOtherwise, it will return the Json value associated with the final key.",26],[11,"search","","If the Json value is an Object, performs a depth-first search until\na value associated with the provided key is found. If no value is found\nor the Json value is not an Object, returns None.",26],[11,"is_object","","Returns true if the Json value is an Object. Returns false otherwise.",26],[11,"as_object","","If the Json value is an Object, returns the associated BTreeMap.\nReturns None otherwise.",26],[11,"is_array","","Returns true if the Json value is an Array. Returns false otherwise.",26],[11,"as_array","","If the Json value is an Array, returns the associated vector.\nReturns None otherwise.",26],[11,"is_string","","Returns true if the Json value is a String. Returns false otherwise.",26],[11,"as_string","","If the Json value is a String, returns the associated str.\nReturns None otherwise.",26],[11,"is_number","","Returns true if the Json value is a Number. Returns false otherwise.",26],[11,"is_i64","","Returns true if the Json value is a i64. Returns false otherwise.",26],[11,"is_u64","","Returns true if the Json value is a u64. Returns false otherwise.",26],[11,"is_f64","","Returns true if the Json value is a f64. Returns false otherwise.",26],[11,"as_i64","","If the Json value is a number, return or cast it to a i64.\nReturns None otherwise.",26],[11,"as_u64","","If the Json value is a number, return or cast it to a u64.\nReturns None otherwise.",26],[11,"as_f64","","If the Json value is a number, return or cast it to a f64.\nReturns None otherwise.",26],[11,"is_boolean","","Returns true if the Json value is a Boolean. Returns false otherwise.",26],[11,"as_boolean","","If the Json value is a Boolean, returns the associated bool.\nReturns None otherwise.",26],[11,"is_null","","Returns true if the Json value is a Null. Returns false otherwise.",26],[11,"as_null","","If the Json value is a Null, returns ().\nReturns None otherwise.",26],[6,"Output","",""],[11,"index","","",26],[6,"Output","",""],[11,"index","","",26],[11,"fmt","","",31],[11,"clone","","",31],[11,"eq","","",31],[11,"ne","","",31],[11,"fmt","","",32],[11,"clone","","",32],[11,"eq","","",32],[11,"ne","","",32],[11,"new","","",36],[11,"len","","Returns The number of elements in the Stack.",36],[11,"is_empty","","Returns true if the stack is empty.",36],[11,"get","","Provides access to the StackElement at a given index.\nlower indices are at the bottom of the stack while higher indices are\nat the top.",36],[11,"is_equal_to","","Compares this stack with an array of StackElements.",36],[11,"starts_with","","Returns true if the bottom-most elements of this stack are the same as\nthe ones passed as parameter.",36],[11,"ends_with","","Returns true if the top-most elements of this stack are the same as\nthe ones passed as parameter.",36],[11,"top","","Returns the top-most element (if any).",36],[6,"Item","",""],[11,"next","","",37],[11,"new","","Creates the JSON parser.",37],[11,"stack","","Provides access to the current position in the logical structure of the\nJSON stream.",37],[11,"new","","Create a JSON Builder.",38],[11,"build","","",38],[11,"new","","Creates a new decoder instance for decoding the specified JSON value.",39],[6,"Error","",""],[11,"read_nil","","",39],[11,"read_usize","","",39],[11,"read_u8","","",39],[11,"read_u16","","",39],[11,"read_u32","","",39],[11,"read_u64","","",39],[11,"read_isize","","",39],[11,"read_i8","","",39],[11,"read_i16","","",39],[11,"read_i32","","",39],[11,"read_i64","","",39],[11,"read_f32","","",39],[11,"read_f64","","",39],[11,"read_bool","","",39],[11,"read_char","","",39],[11,"read_str","","",39],[11,"read_enum","","",39],[11,"read_enum_variant","","",39],[11,"read_enum_variant_arg","","",39],[11,"read_enum_struct_variant","","",39],[11,"read_enum_struct_variant_field","","",39],[11,"read_struct","","",39],[11,"read_struct_field","","",39],[11,"read_tuple","","",39],[11,"read_tuple_arg","","",39],[11,"read_tuple_struct","","",39],[11,"read_tuple_struct_arg","","",39],[11,"read_option","","",39],[11,"read_seq","","",39],[11,"read_seq_elt","","",39],[11,"read_map","","",39],[11,"read_map_elt_key","","",39],[11,"read_map_elt_val","","",39],[11,"error","","",39],[11,"to_json","","",26],[11,"to_json","collections::string","",0],[11,"to_json","collections::vec","",3],[11,"to_json","collections::btree::map","",12],[11,"to_json","std::collections::hash::map","",14],[11,"to_json","core::option","",4],[11,"fmt","rustc-serialize::json","Encodes a json value into a string",26],[11,"fmt","","Encodes a json value into a string",40],[11,"fmt","","Encodes a json value into a string",41],[11,"indent","","Set the indentation level for the emitted JSON",42],[11,"fmt","","Encodes a json value into a string",42],[11,"from_str","","",26],[11,"encode","collections::string","",0],[11,"decode","","",0],[11,"encode","alloc::boxed","",1],[11,"decode","","",1],[11,"decode","","",1],[11,"encode","alloc::rc","",2],[11,"decode","","",2],[11,"encode","collections::vec","",3],[11,"decode","","",3],[11,"encode","core::option","",4],[11,"decode","","",4],[11,"encode","std::path::posix","",5],[11,"decode","","",5],[11,"encode","std::path::windows","",6],[11,"decode","","",6],[11,"encode","core::cell","",7],[11,"decode","","",7],[11,"encode","","",8],[11,"decode","","",8],[11,"encode","alloc::arc","",9],[11,"decode","","",9],[8,"Decoder","rustc-serialize",""],[16,"Error","rustc-serialize::Decoder",""],[10,"read_nil","rustc-serialize","",43],[10,"read_usize","","",43],[10,"read_u64","","",43],[10,"read_u32","","",43],[10,"read_u16","","",43],[10,"read_u8","","",43],[10,"read_isize","","",43],[10,"read_i64","","",43],[10,"read_i32","","",43],[10,"read_i16","","",43],[10,"read_i8","","",43],[10,"read_bool","","",43],[10,"read_f64","","",43],[10,"read_f32","","",43],[10,"read_char","","",43],[10,"read_str","","",43],[10,"read_enum","","",43],[10,"read_enum_variant","","",43],[10,"read_enum_variant_arg","","",43],[10,"read_enum_struct_variant","","",43],[10,"read_enum_struct_variant_field","","",43],[10,"read_struct","","",43],[10,"read_struct_field","","",43],[10,"read_tuple","","",43],[10,"read_tuple_arg","","",43],[10,"read_tuple_struct","","",43],[10,"read_tuple_struct_arg","","",43],[10,"read_option","","",43],[10,"read_seq","","",43],[10,"read_seq_elt","","",43],[10,"read_map","","",43],[10,"read_map_elt_key","","",43],[10,"read_map_elt_val","","",43],[10,"error","","",43],[8,"Encoder","",""],[16,"Error","rustc-serialize::Encoder",""],[10,"emit_nil","rustc-serialize","",44],[10,"emit_usize","","",44],[10,"emit_u64","","",44],[10,"emit_u32","","",44],[10,"emit_u16","","",44],[10,"emit_u8","","",44],[10,"emit_isize","","",44],[10,"emit_i64","","",44],[10,"emit_i32","","",44],[10,"emit_i16","","",44],[10,"emit_i8","","",44],[10,"emit_bool","","",44],[10,"emit_f64","","",44],[10,"emit_f32","","",44],[10,"emit_char","","",44],[10,"emit_str","","",44],[10,"emit_enum","","",44],[10,"emit_enum_variant","","",44],[10,"emit_enum_variant_arg","","",44],[10,"emit_enum_struct_variant","","",44],[10,"emit_enum_struct_variant_field","","",44],[10,"emit_struct","","",44],[10,"emit_struct_field","","",44],[10,"emit_tuple","","",44],[10,"emit_tuple_arg","","",44],[10,"emit_tuple_struct","","",44],[10,"emit_tuple_struct_arg","","",44],[10,"emit_option","","",44],[10,"emit_option_none","","",44],[10,"emit_option_some","","",44],[10,"emit_seq","","",44],[10,"emit_seq_elt","","",44],[10,"emit_map","","",44],[10,"emit_map_elt_key","","",44],[10,"emit_map_elt_val","","",44],[8,"Decodable","",""],[10,"decode","","",45],[8,"Encodable","",""],[10,"encode","","",46],[8,"DecoderHelpers","",""],[10,"read_to_vec","","",47],[8,"EncoderHelpers","",""],[10,"emit_from_vec","","",48]],"paths":[[3,"String"],[3,"Box"],[3,"Rc"],[3,"Vec"],[4,"Option"],[3,"Path"],[3,"Path"],[3,"Cell"],[3,"RefCell"],[3,"Arc"],[3,"DList"],[3,"RingBuf"],[3,"BTreeMap"],[3,"BTreeSet"],[3,"HashMap"],[3,"HashSet"],[3,"VecMap"],[3,"Config"],[4,"CharacterSet"],[4,"Newline"],[4,"FromBase64Error"],[8,"ToBase64"],[8,"FromBase64"],[4,"FromHexError"],[8,"ToHex"],[8,"FromHex"],[4,"Json"],[4,"ErrorCode"],[4,"ParserError"],[4,"DecoderError"],[4,"EncoderError"],[4,"JsonEvent"],[4,"StackElement"],[8,"ToJson"],[3,"Encoder"],[3,"PrettyEncoder"],[3,"Stack"],[3,"Parser"],[3,"Builder"],[3,"Decoder"],[3,"PrettyJson"],[3,"AsJson"],[3,"AsPrettyJson"],[8,"Decoder"],[8,"Encoder"],[8,"Decodable"],[8,"Encodable"],[8,"DecoderHelpers"],[8,"EncoderHelpers"]]};

initSearch(searchIndex);

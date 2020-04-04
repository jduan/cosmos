// this file references "init.st"
// * type and name are expanded directly
// * "<init(value)>" is a call to the "init.st" template, passing the "value"
decl(type, name, value) ::= "<type> <name><init(value)>;"

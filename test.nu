#!/usr/bin/env nu

#test a example
def main [
  id: int # Input the index of the example. ep. 16 when testing 16_*_*.rs
  name = '' # Input the name of test funtion. Default to run all tests.
] {
  let file = ( ls examples | where name =~ ( $id | into string ) 
    | get name.0
    | parse '{dir}\{example}.rs'
    # | get number | get 0
  )
  cargo test $name --example ( $file | get example.0) -- --nocapture --color always
}


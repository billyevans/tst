var searchIndex = {};
searchIndex['tst'] = {"items":[[3,"TSTMap","tst","",null,null],[3,"TSTSet","","A set based on a TSTMap.",null,null],[11,"eq","","",0,{"inputs":[{"name":"tstmap"},{"name":"tstmap"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"tstmap"},{"name":"tstmap"}],"output":{"name":"bool"}}],[11,"clone","","",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"tstmap"}}],[11,"new","","Constructs a new, empty `TSTMap<Value>`.\n# Examples",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"self"}}],[11,"len","","Returns the number of elements in the container.",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"usize"}}],[11,"insert","","Inserts an element at key `key` with value `val`.",0,{"inputs":[{"name":"tstmap"},{"name":"str"},{"name":"value"}],"output":{"name":"option"}}],[11,"entry","","Gets the given key's corresponding entry in the TSTMap for in-place manipulation.",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"entry"}}],[11,"remove","","Removes a key from the TSTMap, returning the value at the key if the key\nwas previously in the TSTMap.",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"option"}}],[11,"get","","Returns a reference to the value corresponding to the key or None.",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"option"}}],[11,"get_mut","","Returns a mutable reference to the value corresponding to the key.",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"option"}}],[11,"contains_key","","Returns true if the TSTMap contains a value for the specified key.\n# Examples",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"bool"}}],[11,"is_empty","","Returns true if the TSTMap contains no elements.",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"bool"}}],[11,"clear","","Clears the TSTMap.",0,{"inputs":[{"name":"tstmap"}],"output":null}],[11,"wildcard_iter","","An iterator returning all nodes matching wildcard pattern.\nIterator element type is (String, V)",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"wildcarditer"}}],[11,"wildcard_iter_mut","","An mutable iterator returning all nodes matching wildcard pattern.",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"wildcarditermut"}}],[11,"prefix_iter","","Method returns iterator over all values with common prefix in the TSTMap\n# Examples",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"iter"}}],[11,"prefix_iter_mut","","Method returns mutable iterator over all values with common prefix in the TSTMap\n# Examples",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"itermut"}}],[11,"iter","","Gets an iterator over the entries of the TSTMap.",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"iter"}}],[11,"iter_mut","","Gets a mutable iterator over the entries of the TSTMap.",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"itermut"}}],[11,"keys","","An iterator visiting all keys in arbitrary order.\nIterator element type is String",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"keysiter"}}],[11,"values","","An iterator visiting all values in arbitrary order.\nIterator element type is &V",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"valuesiter"}}],[11,"longest_prefix","","Method returns longest prefix in the TSTMap",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"str"}}],[11,"into_iter","","Creates a consuming iterator, that is, one that moves each key-value\npair out of the TSTMap in arbitrary order. The TSTMap cannot be used after\ncalling this.",0,{"inputs":[{"name":"tstmap"}],"output":{"name":"intoiter"}}],[11,"from_iter","","",0,{"inputs":[{"name":"tstmap"},{"name":"i"}],"output":{"name":"tstmap"}}],[11,"extend","","",0,{"inputs":[{"name":"tstmap"},{"name":"i"}],"output":null}],[11,"index","","",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"value"}}],[11,"index_mut","","",0,{"inputs":[{"name":"tstmap"},{"name":"str"}],"output":{"name":"value"}}],[11,"drop","","",0,{"inputs":[{"name":"tstmap"}],"output":null}],[11,"fmt","","",0,{"inputs":[{"name":"tstmap"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"tstset"},{"name":"tstset"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"tstset"},{"name":"tstset"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"tstset"}],"output":{"name":"tstset"}}],[11,"new","","Makes a new empty TSTSet.",1,{"inputs":[{"name":"tstset"}],"output":{"name":"self"}}],[11,"len","","Returns the number of elements in the set.",1,{"inputs":[{"name":"tstset"}],"output":{"name":"usize"}}],[11,"is_empty","","Returns true if the set contains no elements.",1,{"inputs":[{"name":"tstset"}],"output":{"name":"bool"}}],[11,"clear","","Clears the set, removing all values.",1,{"inputs":[{"name":"tstset"}],"output":null}],[11,"contains","","Returns `true` if the set contains a value.",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"bool"}}],[11,"insert","","Adds a value to the set.",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"bool"}}],[11,"remove","","Removes a value from the set. Returns `true` if the value was\npresent in the set.",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"bool"}}],[11,"iter","","Gets an iterator over the TSTSet's contents.",1,{"inputs":[{"name":"tstset"}],"output":{"name":"iter"}}],[11,"wildcard_iter","","An iterator returning all nodes matching wildcard pattern.\nIterator element type is (String)",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"wildcarditer"}}],[11,"longest_prefix","","Method returns longest prefix in the TSTSet.",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"str"}}],[11,"prefix_iter","","Method returns iterator over all values with common prefix in the TSTSet.\n# Examples",1,{"inputs":[{"name":"tstset"},{"name":"str"}],"output":{"name":"iter"}}],[11,"into_iter","","Creates a consuming iterator, that is, one that moves each key-value\npair out of the TSTMap in arbitrary order. The TSTMap cannot be used after\ncalling this.",1,{"inputs":[{"name":"tstset"}],"output":{"name":"intoiter"}}],[11,"from_iter","","",1,{"inputs":[{"name":"tstset"},{"name":"i"}],"output":{"name":"tstset"}}],[11,"extend","","",1,{"inputs":[{"name":"tstset"},{"name":"i"}],"output":null}],[11,"fmt","","",1,{"inputs":[{"name":"tstset"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"tst_map","","",null,null],[3,"TSTMap","tst::tst_map","",null,null],[3,"Iter","","TSTMap iterator.",null,null],[3,"IterMut","","TSTMap mutable iterator.",null,null],[3,"KeysIter","","TSTMap keys iterator",null,null],[3,"ValuesIter","","TSTMap values iterator",null,null],[3,"WildCardIter","","TSTMap wild-card iterator.",null,null],[3,"WildCardIterMut","","TSTMap wild-card mutable iterator.",null,null],[3,"IntoIter","","TSTMap consuming iterator",null,null],[3,"OccupiedEntry","","A view into a single occupied location in a TSTMap.",null,null],[3,"VacantEntry","","A view into a single empty location in a TSTMap.",null,null],[4,"Entry","","A view into a single location in a TSTMap, which may be vacant or occupied.",null,null],[13,"Occupied","","A vacant Entry",2,null],[13,"Vacant","","An occupied Entry",2,null],[0,"tst_set","tst","",null,null],[3,"TSTSet","tst::tst_set","A set based on a TSTMap.",null,null],[3,"Iter","","An iterator over a TSTSet's items.",null,null],[3,"IntoIter","","An owning iterator over a TSTSet's items.",null,null],[3,"WildCardIter","","TSTMap wild-card iterator.",null,null],[14,"tstmap!","tst","- Create a `TSTMap` containing a given list of elements:",null,null],[14,"tstset!","","- Create a `TSTSet` containing a given list of elements:",null,null],[11,"default","tst::tst_map","",3,{"inputs":[{"name":"iter"}],"output":{"name":"iter"}}],[11,"clone","","",3,{"inputs":[{"name":"iter"}],"output":{"name":"iter"}}],[11,"next","","",3,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"size_hint","","",3,null],[11,"default","","",4,{"inputs":[{"name":"itermut"}],"output":{"name":"itermut"}}],[11,"clone","","",4,{"inputs":[{"name":"itermut"}],"output":{"name":"itermut"}}],[11,"next","","",4,{"inputs":[{"name":"itermut"}],"output":{"name":"option"}}],[11,"size_hint","","",4,null],[11,"clone","","",5,{"inputs":[{"name":"keysiter"}],"output":{"name":"keysiter"}}],[11,"next","","",5,{"inputs":[{"name":"keysiter"}],"output":{"name":"option"}}],[11,"size_hint","","",5,null],[11,"clone","","",6,{"inputs":[{"name":"valuesiter"}],"output":{"name":"valuesiter"}}],[11,"next","","",6,{"inputs":[{"name":"valuesiter"}],"output":{"name":"option"}}],[11,"size_hint","","",6,null],[11,"clone","","",7,{"inputs":[{"name":"wildcarditer"}],"output":{"name":"wildcarditer"}}],[11,"next","","",7,{"inputs":[{"name":"wildcarditer"}],"output":{"name":"option"}}],[11,"size_hint","","",7,null],[11,"clone","","",8,{"inputs":[{"name":"wildcarditermut"}],"output":{"name":"wildcarditermut"}}],[11,"next","","",8,{"inputs":[{"name":"wildcarditermut"}],"output":{"name":"option"}}],[11,"size_hint","","",8,null],[11,"next","","",9,{"inputs":[{"name":"intoiter"}],"output":{"name":"option"}}],[11,"size_hint","","",9,null],[11,"len","","",9,{"inputs":[{"name":"intoiter"}],"output":{"name":"usize"}}],[11,"get","","Gets a mut reference to the value in the entry or Err in case for Vacant.",2,{"inputs":[{"name":"entry"}],"output":{"name":"result"}}],[11,"or_insert","","Ensures a value is in the entry by inserting the default if empty, and returns\na mutable reference to the value in the entry.",2,{"inputs":[{"name":"entry"},{"name":"value"}],"output":{"name":"value"}}],[11,"or_insert_with","","Ensures a value is in the entry by inserting the result of the default function if empty,\nand returns a mutable reference to the value in the entry.",2,{"inputs":[{"name":"entry"},{"name":"f"}],"output":{"name":"value"}}],[11,"get","","Gets a reference to the value in the entry.",10,{"inputs":[{"name":"occupiedentry"}],"output":{"name":"value"}}],[11,"get_mut","","Gets a mutable reference to the value in the entry.",10,{"inputs":[{"name":"occupiedentry"}],"output":{"name":"value"}}],[11,"into_mut","","Converts the OccupiedEntry into a mutable reference to the value in the entry\nwith a lifetime bound to the TSTMap itself",10,{"inputs":[{"name":"occupiedentry"}],"output":{"name":"value"}}],[11,"insert","","Sets the value of the entry, and returns the entry's old value",10,{"inputs":[{"name":"occupiedentry"},{"name":"value"}],"output":{"name":"value"}}],[11,"remove","","Takes the value out of the entry, and returns it",10,{"inputs":[{"name":"occupiedentry"}],"output":{"name":"value"}}],[11,"insert","","Sets the value of the entry with the VacantEntry's key,\nand returns a mutable reference to it",11,{"inputs":[{"name":"vacantentry"},{"name":"value"}],"output":{"name":"value"}}],[11,"clone","tst::tst_set","",12,{"inputs":[{"name":"iter"}],"output":{"name":"iter"}}],[11,"clone","","",13,{"inputs":[{"name":"wildcarditer"}],"output":{"name":"wildcarditer"}}],[11,"next","","",12,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"size_hint","","",12,null],[11,"next","","",14,{"inputs":[{"name":"intoiter"}],"output":{"name":"option"}}],[11,"size_hint","","",14,null],[11,"len","","",14,{"inputs":[{"name":"intoiter"}],"output":{"name":"usize"}}],[11,"next","","",13,{"inputs":[{"name":"wildcarditer"}],"output":{"name":"option"}}],[11,"size_hint","","",13,null]],"paths":[[3,"TSTMap"],[3,"TSTSet"],[4,"Entry"],[3,"Iter"],[3,"IterMut"],[3,"KeysIter"],[3,"ValuesIter"],[3,"WildCardIter"],[3,"WildCardIterMut"],[3,"IntoIter"],[3,"OccupiedEntry"],[3,"VacantEntry"],[3,"Iter"],[3,"WildCardIter"],[3,"IntoIter"]]};
initSearch(searchIndex);

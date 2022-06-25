window.SIDEBAR_ITEMS = {"fn":[["from_wire","Deserialize a `T` from some bytes and, optionally, some handles. If the wire is not able to transport handles, it is fine to produce `&[]` (as long as `T` does not contain any handles, that is)."],["index_from_handle_slot",""],["make_handle_slot",""],["serialized_size","It can sometimes be useful to know the size of a value in its serialized form (e.g. to reserve space for it in a ring buffer). This calculates the number of bytes taken to serialize some `value` of `T` into Ptah’s wire format. Note that this size is for the specific `value`, and may differ between values of `T`."],["to_wire",""]],"mod":[["de",""],["ser",""]],"struct":[["CursorWriter","This is a `Writer` that can be used to serialize a value into a pre-allocated byte buffer."]],"trait":[["Writer","A `Writer` represents a consumer of the bytes produced by serializing a message. In cases where you can create a slice to put the bytes in, `CursorWriter` can be used. Custom `Writer`s are useful for more niche uses, such as sending the serialized bytes over a serial port."]],"type":[["Handle",""],["HandleSlot",""]]};
# Settings
``` py linenums="1"
setting {
    index-all=True # Make structures, enums, errors protocols indexing explicit by default
    index-structs=True # Make structures indexing explicit by default
    index-enums=True # Make enums indexing explicit by default
    index-errors=True # Make errors indexing explicit by default
    index-protocols=True # Make protocols indexing explicit by default

    forbid-default-index-all=False # Forbid indexing explicit for any component by default
    forbid-default-index-structs=False # Forbid indexing explicit for structs by default
    forbid-default-index-enums=False # Forbid indexing explicit for enums by default
    forbid-default-index-errors=False # Forbid indexing explicit for errors by default
    forbid-default-index-protocols=False # Forbid indexing explicit for protocols by default
    
    forbid-index-all=False # Forbid indexing entirely for any component by default
    forbid-index-structs=False # Forbid indexing entirely for structs by default
    forbid-index-enums=False # Forbid indexing entirely for enums by default
    forbid-index-errors=False # Forbid indexing entirely for errors by default
    forbid-index-protocols=False # Forbid indexing entirely for protocols by default
}
```
Values for each **parameter** above are the **default** value, which you
can change to affect the whole schema


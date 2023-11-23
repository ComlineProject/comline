# Structure
A **structure** (also called **struct** or **message**) defines
the properties of the data carried and set on their **fields**,
for each field, they also have properties of their own like **index**,
**type** and **default** value

``` py linenums="1"
struct Message {
    1# sender: str = "Bee"
    2# deliver: Deliver = Deliver::To
    3# recipient: str = "Flower"
}
```

## Field
A field has a index, a **name** (also called **id**), a **type**, and
optionally a **default** value
```py linenums="1"
1# optional sender: str = "Bee"
```


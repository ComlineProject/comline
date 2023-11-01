# Dynamic Languages Codegen


## Generated Code
Generates the respective code for the languages:

=== "Python"

    ```py
    class Message(Protocol):
      receiver: string
      subject: string
      message: string
    ```

=== "TypeScript"

    ```ts
    interface Message {
      receiver: string
      subject: string
      message: string
    }
    ```

=== "Luau"

    ```luau
    type Message = {
        receiver: string,
        subject: string,
        message: string
    }
    ```


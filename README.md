# Quokka-interpeter
a custom interpeter implementation using Thorsten ball book.
  https://interpreterbook.com/

## Use the Interperter(REPL)
clone the project:
```
git clone https://github.com/ImTheCurse/Quokka-interpeter.git
```
cd to directory:
```
cd Quokka-interpeter
```
compile and run:
```
cargo run
```

### Table of Contents
- [Syntax overview](#syntax-overview)
- [If](#if)
- [Operators](#operators)
- [Variables](#variables)
- [Return](#return)
- [Functions](#functions)
- [Literals](#literals)

# syntax-overview
```
let add = fn(x,y){
return x + y;
}
add(5,3); // we will get here 8.
```

#### If
Supports if + else but dosent support else if.
```
if(true){
  //do something
}else{
  //do alternative
}
```
#### Operators
Supports +, -, !, / , *, '' operators.
```
1 + 2 + (3 * 4) - (10 / 5);
!true;
!false;
-5;
'Hello' + ' ' + 'World';
```

#### Variables
We define variables using the let keyword.
Foramt:
```
let <identifier> = <value>
```
```
let x = 5;
```
#### Return
returns the value as expected, can be used inside a block / function.
  ```
    let x = fn(){
      return 5;
    }
    x();
  ```

#### Functions
as seen before, we use functions in the following format:
```
fn (<parameter one>, <parameter two>, ...) { <block statement> };
```
```
let x = fn(){
      return 5;
    }
```

#### Literals
there are currently 3 data type which are: integer, boolen and strings, but I intend to add more in the near future.
Integer:
```
1;
5;
let y = 7;
```
Boolen:
```true | false```
```
let x = true;
if(x){
  //always execute this
}
```
String:
```
'<value>'
```
```
'hello world!'
```





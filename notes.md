## Representation of Data
- in computing, we care about storing and transferring data or information
- there are different types of data we may want to transfer
    * boolean
    * integers
    * text
    * structured data (XML, JSON)
    * binary (image)
- programming languages help us work with these formats

## Two types of computing
- analog systems
- digital systems
    * communication using stream of 0s and 1s
    * each piece of information is called bit
    * we can store bits in memory or transmit them
- modern computers are primarily digital systems

## Bits
- a single but is the smallest piece of data in a digital system
- it can be either 0 or 1
- a single bit is not enough to encode a single number

## Multiple Bits
- to make working bits easier, we put several of them together
- a single bit can only encode 2 state, (0, 1)
- but two bits together can encode 4 state (00, 01, 10, 11)
- so, n bits allow us to encode 2n state that allow us to represent whole numbers

## Bytes
- the most common value of n is 8, which allow us to encode 2power8 (256 states), and its called a byte
- some example to use a byte, like storing age, represent every single symbol in latin based alphabet, etc
- we can put many bytes together to store bigger state of data

## How to use the Bits?
- to store whole non-negative numbers unsigned and signed

## Platform specific types
- cpu and process have bitness, 32-bit CPU, 64-bit CPU
- the bitness puts a limit on range of memory you can access, so 32-bit can only use up to 4Gb RAM
- many programming languanges provide platform-specific integer types
- these types are useful when you need a general-purpose integer variable or need to access an array element

## Floating point numbers
- are used to store non-whole value (1.234, -5.00001)
- there are two data types, 32-bit (float/single-precision), 64 (double/double-precision)
- floating-points representations is standardized (IEEE754)
- not allow to extract representation of numbers
- represent a range of special values, infinity, NaNs
# Glossary

## code

The code being returned in a *[initcode](#initcode) of a *create* transaction,
`CREATE`, or `CREATE2` instructions.

## initcode

Code executed in the context of *create* transaction, `CREATE`, or
`CREATE2` instructions.

The *initcode* returns *[code](#code)* (via the `RETURN` instruction), which is
inserted into the account.

See section 7 ("Contract Creation" in the [Yellow
Paper](https://ethereum.github.io/yellowpaper/paper.pdf) for more information.

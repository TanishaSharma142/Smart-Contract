#Programs don't hold data: 
In Solana, the smart contract (Program) is just the logic/code. It doesn't store any info inside itself. Instead, we create a separate "Account" (like a storage box) to actually save the data, such as who owns the vault.

#PDAs are just "Program Wallets": 'PDA = ghost wallet'
A PDA (Program Derived Address) is basically a wallet that belongs to the code, not a person. It has no private key, which is great for security because it means no human can "sign" to steal the money. Only the specific code in my program can move funds out of it.

#Seeds are like coordinates: 'seed = passward'
To find that PDA wallet, we don't memorize a random address. We use "seeds" (like a combination of the word "state" + my wallet address). This acts like a map coordinate—the program mixes them together to find the exact same vault location every single time.

#Transactions are "All or Nothing": 
Solana is strict. If any part of my code fails (like if someone tries to withdraw 10 SOL but only has 5), the entire transaction is cancelled immediately. It’s like an "undo" button; the money never leaves the wallet unless everything is perfect.

#Rent is a storage fee: 
Storing data on the blockchain isn't free. We pay a tiny deposit called "rent" (in SOL) to keep the account open. It’s just like paying for a digital storage locker.

successfully created smart contract
<img width="1330" height="538" alt="image" src="https://github.com/user-attachments/assets/936b9fac-d9b5-46c2-b530-0f0f839723b4" />

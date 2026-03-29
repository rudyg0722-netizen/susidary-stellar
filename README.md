

# 🌾 AgriSubsidyManager

A Soroban smart contract deployed on the Stellar **testnet** that enables transparent, secure, and efficient distribution of agricultural subsidies.

🔗 **Live Contract (Testnet)**
👉 [https://stellar.expert/explorer/testnet/contract/CBDOCJUOIXE5MJ7MXBS3APWZ2GSBTYLLMBN4XFPI6Z4AABSMUEW27L2E](https://stellar.expert/explorer/testnet/contract/CBDOCJUOIXE5MJ7MXBS3APWZ2GSBTYLLMBN4XFPI6Z4AABSMUEW27L2E)

---

## 🚀 Overview

AgriSubsidyManager is designed to solve real-world inefficiencies in subsidy distribution by:

* Eliminating middlemen
* Ensing direct benefit transfer (DBT)
* Providing on-chain transparency
* Enabling farmers to claim funds securely

---

## ⚙️ Features

### 🔐 Admin-Controlled Distribution

* Only the authorized admin can release subsidies
* Prevents unauthorized fund allocation

### 💰 Farmer Claim Mechanism

* Farmers can claim allocated subsidies directly
* Funds are reset after claim (prevents double spending)

### 📊 On-Chain Transparency

* All allocations stored on-chain
* Anyone can verify subsidy balances

### 📡 Event Logging

* Emits events for:

  * Subsidy release
  * Subsidy claim

---

## 🧠 Contract Architecture

### Core Functions

#### 1. `init(admin: Address)`

Initializes the contract with an admin.

#### 2. `release(farmer: Address, amount: i128)`

* Called by admin
* Allocates subsidy to a farmer
* Accumulates if already exists

#### 3. `claim(farmer: Address)`

* Called by farmer
* Transfers (logically) and resets subsidy to 0

#### 4. `get_subsidy(farmer: Address)`

* View function
* Returns current subsidy balance

---

## 🏗 Tech Stack

* **Smart Contracts**: Rust + Soroban SDK
* **Blockchain**: Stellar
* **Network**: Testnet
* **Storage**: Instance storage (key-value)

---

## 🔄 Flow

1. Admin initializes contract
2. Admin releases subsidy to farmer
3. Farmer checks balance
4. Farmer claims subsidy
5. Balance resets to zero

---

## 🧪 Example Usage

### Release Subsidy

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- \
  release \
  --farmer <FARMER_ADDRESS> \
  --amount 1000
```

### Claim Subsidy

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source farmer \
  --network testnet \
  -- \
  claim \
  --farmer <FARMER_ADDRESS>
```

---

## 🔐 Security Considerations

* Requires authentication for both admin and farmer
* Prevents unauthorized claims
* Avoids double claiming via reset mechanism

---

## 🌍 Real-World Use Cases

* Government subsidy programs
* NGO financial aid distribution
* Crop insurance payouts
* Fertilizer / irrigation grants

---

## 🚀 Future Improvements

* 🧾 Multi-subsidy schemes (fertilizer, seeds, irrigation)
* ⏳ Time-locked subsidies
* 🆔 Farmer identity via NFTs
* 📱 Frontend dashboard (React + Stellar wallet)
* 🛰 Integration with IoT / agri-data

---

## 🤝 Contributing

Feel free to fork, improve, and build on top of this contract.

---


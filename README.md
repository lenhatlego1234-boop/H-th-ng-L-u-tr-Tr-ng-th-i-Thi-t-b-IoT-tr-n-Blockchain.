# IoT Device Tracker

## Project Title
IoT Device Tracker

## Project Description
IoT Device Tracker is a decentralized smart contract platform designed to securely log and manage the operational states of physical hardware devices (such as ESP32, Arduino, and smart sensors). Built using the Soroban SDK on the Stellar blockchain, it provides a transparent and immutable registry where device statuses (e.g., ON/OFF, active/inactive) are recorded trustlessly, bridging the gap between embedded systems and Web3.

## Project Vision
The vision of IoT Device Tracker is to eliminate single points of failure and data manipulation in traditional IoT ecosystems. By leveraging decentralized ledger technology, it offers hardware developers and enterprises a tamper-proof, publicly verifiable database to track physical assets, ensuring maximum transparency, security, and reliability for smart home networks, industrial controllers, and sensor grids.

## Key Features
- **Device State Management:** Seamlessly update the operational status of any registered IoT node.
- **Immutable Records:** All hardware state changes are permanently recorded on-chain, creating a transparent audit trail.
- **Real-time Querying:** Anyone can query the current status of a specific device ID securely and instantly.
- **Lightweight Architecture:** Optimized for low-resource environments, making it ideal for microcontrollers.
- **Decentralized Storage:** Utilizes Soroban's Instance Storage to ensure data persistence without relying on centralized servers.

## Usage Instructions
1. **Deploy Contract:** Deploy the smart contract to the Stellar network.
2. **Define Device ID:** Assign a unique `Symbol` identifier to your physical device (e.g., `SENSOR_1`, `ROOM_RELAY`).
3. **Set Status:** Call the `set_status` function, passing the device ID and its current state (`true` for ON/Active, `false` for OFF/Inactive).
4. **Query Status:** Use the `get_status` function to retrieve the real-time operational state of the device on-chain.

## Future Scope
- **Hardware Integration APIs:** Develop lightweight C/C++ libraries to allow modules to push updates directly to the Stellar network.
- **Access Control:** Implement cryptographic signature verification so only authorized device owners can update the hardware state.
- **Advanced Sensor Data Logging:** Expand storage capabilities to record analog sensor values (e.g., temperature, humidity) alongside binary states.
- **Automated Triggers:** Integrate with smart contract oracles to trigger on-chain events based on physical sensor inputs.
- **Historical State Tracking:** Add event-emitting features to track the complete history of a device's uptime and downtime.

## Technology Stack
- **Rust and Soroban SDK** (v25) for secure, highly optimized smart contract development.
- **Stellar Blockchain** for fast, low-cost, and decentralized state management.
- **Embedded C/C++** (planned) for physical node clients.

## Contribution
Community contributions are highly welcomed from both blockchain developers and embedded systems engineers. Fork the repository and submit pull requests to help expand the bridge between physical hardware and Web3.

## License
This project is licensed under the MIT License.

### Contract Detail
ID: CDJ5XVXDLDGUAW3H5CAKZXIURM64TNPXMVZGG6EZOHPVXHUGZNBLWGQK

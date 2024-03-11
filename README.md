# Hack Secret 2024

## Problem Statement
In today's world, the media landscape is often dominated by major political figures and wealthy individuals who can easily manipulate the information presented to the public through news channels. This control over the narrative can lead to biased or even false reporting, undermining the trust in media and hindering the public's access to accurate information.

## Proposed Solution
To address the issue of biased and manipulated news reporting, we have developed a blockchain-based news posting application. This application relies on user feedback for authenticity, ensuring that news articles are verified by both artificial intelligence screening and public voting. The platform nowhere uses even the wallet address of any creator aiming to promote transparency, authenticity, and freedom of expression in media.

## Development Deepdive

Contracts are written in `Rust` for it's robustness and speed which are them compiled to `WASM` (Web Assembly) making interactions possible through web. The contracts are being deployed on Secret Network. For UI we are using `NextJS` as it provides both SSR and SSG making the interface secure and fast at the same time.

For compiling the contract and terminal based interactions for query and execution of commands, setup instructions and guidelines are provided in the subfolder [here](https://github.com/KarthikS373/signal/tree/main/contract#readme). (Includes instructions for both Windows and Linux/Mac)

Major Functionalities:
- Post news (Uploadded to IPFS)
- Query news
- Create creator profile
- Create validator profile
- Withdraw Tip
- Stake amount
- Unstake amount

 A user-friendly mobile responsive frontend: https://signal-kohl.vercel.app/

Preview:
Homepage
![image](https://github.com/KarthikS373/signal/assets/31801256/c1416487-ef28-4549-9fa7-6b1651f70730)
Read news article
![image](https://github.com/KarthikS373/signal/assets/31801256/45fe3029-9849-4aeb-a0b0-c383ab277e7f)
Post news
![image](https://github.com/KarthikS373/signal/assets/31801256/cba828a4-44ed-4a4c-85f1-65fa96ae81ae)
Profile page
![image](https://github.com/KarthikS373/signal/assets/31801256/50631a28-b21d-4f83-9260-421bacdf74ad)

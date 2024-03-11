# Hack Secret 2024

The current media landscape faces a crisis of trust. Journalists fear exposure due to leaks, jeopardizing investigative reporting. Malicious actors exploit anonymity to spread misinformation disguised as legitimate news. The public passively consumes information, lacking tools to discern truth from falsehood. Signal envisions a future where journalism thrives in a safe, anonymous environment by using AI and community involvement. Signal gives community the chance to become active participants in the fight against fake news. We aim to cultivate a platform built on trust and transparency, where journalists can report freely and the public can access reliable information.

## Problem Statement
### A Broken News Landscape
The current media environment is riddled with issues that threaten both journalists and the public's right to truthful information

- **Journalists in danger**: When journalists' identities are revealed, it can be dangerous for them. This makes them scared to report on important stories and the public misses out on important information.
- **Fake news**: People who want to spread lies pretend to be real journalists. This makes it hard to know what news to trust and can trick people into believing wrong information.
- **People just take what they hear**: The way people get the news right now doesn't give them much control over what's true and what's not. This makes it easy for false information to spread without anyone stopping it

## Vision
**Signal** envisions a media landscape transformed by trust and empowered by its users. Here's the core of our vision:

- **Safe haven for journalists**: We strive for a world where journalists feel secure sharing crucial information, free from fear of reprisal. Anonymity on Signal allows them to report without risking their safety
- **Eradicating the fake news epidemic**: Through a combination of AI and community involvement, Signal aims to be a fortress against misinformation
- **Active news consumers, not passive bystanders**: Our platform cultivates an interactive experience where users are no longer passive recipients of information. They become active participants by voting on stories, flagging suspicious content and working together to ensure only truth prevails
- **A foundation of trust and transparency**: Our vision is to cultivate a media environment where both journalists and the public can operate with confidence. Transparency in news sources and a platform built on user trust paves the way for a more informed and empowered society
  

## Signal
Signal is a revolutionary news platform designed to empower journalists and users alike in the fight for truthful news

### Features:
- **Anonymous Platform**: Journalists can share information with complete anonymity, protecting their safety and encouraging investigative reporting
- **AI-Powered Validation**: Advanced AI scans incoming stories for potential inaccuracies and inconsistencies acting as a first line of defense against misinformation
- **Community Review System**: Suspicious stories can be flagged for further investigation by the Signal community, harnessing collective intelligence to identify and remove fake news
- **User Voting**: Upvote credible stories and downvote questionable ones to promote better content discovery and surface reliable information
- **Reward System**: Users earn recognition for contributing to truth verification through flagging and voting. Journalists can receive anonymous tips in appreciation for their work

## Development Deepdive

Contracts are written in `Rust` for it's robustness and speed which are them compiled to `WASM` (Web Assembly) making interactions possible through web. The contracts are being deployed on Secret Network. For UI we are using `NextJS` as it provides both SSR and SSG making the interface secure and fast at the same time.

For compiling the contract and terminal based interactions for query and execution of commands, setup instructions and guidelines are provided in the subfolder [here](https://github.com/KarthikS373/signal/tree/main/contract#readme). (Includes instructions for both Windows and Linux/Mac)

---

Frontend: https://signal-kohl.vercel.app/

Preview:

Homepage:
![image](https://github.com/KarthikS373/signal/assets/31801256/c1416487-ef28-4549-9fa7-6b1651f70730)
Read news article:
![image](https://github.com/KarthikS373/signal/assets/31801256/45fe3029-9849-4aeb-a0b0-c383ab277e7f)
Post news:
![image](https://github.com/KarthikS373/signal/assets/31801256/cba828a4-44ed-4a4c-85f1-65fa96ae81ae)
Profile page:
![image](https://github.com/KarthikS373/signal/assets/31801256/50631a28-b21d-4f83-9260-421bacdf74ad)

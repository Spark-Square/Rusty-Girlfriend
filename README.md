# 💕 AI Girlfriend App

An experimental web app that lets you chat with an AI girlfriend 👰🏻‍♀️💖

Built with **Rust** on the backend and **WebAssembly (Yew)** on the frontend.  






## ✨ Features
- 💬 Chat with an AI in real time  
- 🖥️ Runs directly in your browser  
> *Minimum features for now — more coming soon!*







## 🚀 Getting Started

### 1. Install Dependencies
You’ll need the following installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install) (for backend)  
- [Trunk](https://trunkrs.dev) (to build the frontend)  

---

### 2. Clone the Repository
```bash
git clone https://github.com/Spark-Square/Rusty-Girlfriend.git
cd Rusty-Girlfriend
```
---

### 3. Add Your API Key
The backend needs an AI Horde API key to generate responses. 

Create a `.env` file inside the `backend/` folder:  
```text
AI_HORDE_API_KEY=your_api_key_here
```
Replace ```your_api_key_here``` with your personal AI Horde key.

You can register for a free key here: [AI Horde Registration](https://aihorde.net/)

Or, for a public key, you can use ```0000000000``` (10 zeros).

Note: public key responses are slower and may be rate-limited.

----

### 4. Build Frontend

```bash
cd frontend
trunk build --release
```

This will compile the Yew frontend to WebAssembly and produce the dist/ folder

---

### 5. Run Backend Server

```bash
cd ../backend
cargo run
```

The Rocket backend will start and serve the frontend as well as handle chat requests.

---
### 6. Open in Your Browser

Visit:

http://localhost:8000

---

## 🎯 Goals

This project is a personal experiment in building an AI girlfriend app. Current and future goals include:

- 🧠 Improve AI conversation quality and context retention  
- 🌐 Make the app fully usable in the browser without extra setup  
- 🖥️ Integrate with Tauri for a desktop application  
- 💾 Add database support to save chat history  
- 🎨 Improve the UI/UX and styling of the chat interface  
- ⚡ Optimize performance and reduce latency  

> These are ongoing goals — the project is a work in progress!

## 🛠️ Development Notes

Frontend is built with Yew + WebAssembly

Backend is built with Rocket in Rust

API requests are made to AI Horde for text generation

The plan is to add SurrealDB for chat history and Tauri for desktop app

Then the project with use the full Rust RSTY stack 

## ❤️ Contributing

This is an experimental project — ideas, feedback, and pull requests are always welcome!
## 📜 License

© 2025 Spark Square

Licensed under the GPL-3.0 License

<div align="center">
<div align="center">
  <img src="images/logo.png" alt="Lechapes Logo" width="120">
</div>
# JHotel Desktop

### Modern Hotel Management Application

**A desktop application designed to simplify hotel operations, organize guest information, and manage room occupancy efficiently.**

![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust)
![egui](https://img.shields.io/badge/Interface-egui-blue)
![Platform](https://img.shields.io/badge/Platform-Windows-lightgrey?logo=windows)
![Status](https://img.shields.io/badge/Project-Development-yellow)

</div>

---

Designed with simplicity and usability in mind, JHotel helps hotel staff keep essential information organized while providing a clear overview of daily hotel activities.

## Overview

**JHotel** is a desktop hotel management application developed in Rust. It provides a centralized interface for managing rooms, guests, reservations, and occupancy information.

The application combines a modern graphical interface with local data persistence to deliver a practical hotel management experience.

## Features

### Dashboard
- Overview of important hotel information.
- Quick access to the application's main sections.
- Date and time display.
- Daily activity indicators.

### Room Management
- Manage hotel rooms.
- View room information and availability.
- Identify available, occupied, and reserved rooms.
- Organize rooms according to their information and status.

### Guest Management
- Register and manage guest information.
- View guest details.
- Keep guest records organized.
- Access guest information when managing reservations and stays.

### Reservation & Occupancy Management
- Manage hotel reservations.
- Record check-in information and length of stay.
- Track room occupancy.
- Distinguish between available, reserved, and occupied rooms.
- Monitor ongoing and completed stays.

### Search
- Search through hotel information.
- Quickly retrieve relevant records.
- Access information through a dedicated search interface.

### Settings & Personalization
- Switch between light and dark themes.
- Change the application language.
- Customize the application identity.
- Select a custom logo.
- Save application preferences.

## Interface

JHotel is designed around a clean and structured desktop interface.

The application focuses on:
- Clear navigation.
- Consistent visual elements.
- Readable information.
- Organized management screens.
- A customizable appearance.

##️ Technology Stack

| Technology | Purpose |
|---|---|
| **Rust** | Core application development |
| **egui** | Immediate-mode graphical user interface |
| **eframe** | Desktop application framework |
| **SQLite / rusqlite** | Database operations |
| **Serde** | Data serialization and deserialization |
| **Chrono** | Date and time handling |
| **Image** | Image loading and processing |

## Project Structure

```text
JHotel/
│
├── .cargo/
│
├── images/
│   ├── Application icons
│   ├── Navigation images
│   ├── Logo and visual assets
│   └── Other interface images
│
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── donnees.rs
│   ├── modele.rs
│   ├── persistance.rs
│   ├── theme.rs
│   ├── email.rs
│   ├── vue_chambres.rs
│   ├── vue_clients.rs
│   ├── vue_occupations.rs
│   └── vue_recherche.rs
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### Source Code Organization

| File | Responsibility |
|---|---|
| `main.rs` | Application entry point |
| `app.rs` | Main application structure and navigation |
| `donnees.rs` | Hotel data and business operations |
| `modele.rs` | Application data models |
| `persistance.rs` | Data persistence |
| `theme.rs` | Interface styling, themes, images and language |
| `email.rs` | Email-related functionality |
| `vue_chambres.rs` | Room management interface |
| `vue_clients.rs` | Guest management interface |
| `vue_occupations.rs` | Occupancy management interface |
| `vue_recherche.rs` | Search interface |

## Getting Started

### Prerequisites

Before working with the source code, make sure you have:

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)
- A compatible Windows development environment

### Clone the Repository

```bash
git clone https://github.com/Johanes-mg/JHotelDesktop.git
cd JHotel
```

### Install Dependencies

Cargo automatically resolves the dependencies declared in `Cargo.toml`.

```bash
cargo check
```

### Run the Application

```bash
cargo run
```

### Build the Project

```bash
cargo build --release
```

## Design Philosophy

JHotel follows a simple design philosophy:

- **Clarity:** Information should be easy to understand.
- **Efficiency:** Common operations should remain accessible.
- **Consistency:** Interface elements should behave consistently.
- **Flexibility:** Users should be able to personalize their workspace.
- **Maintainability:** The codebase is separated into dedicated modules.

## Data & Persistence

JHotel includes a dedicated persistence layer to manage application data.

The project uses Rust serialization tools and database-related components to support structured data management.

Users should maintain regular backups of their hotel information to reduce the risk of accidental data loss.

## Roadmap

Potential areas for future improvements include:

- [ ] Enhanced reporting and statistics.
- [ ] More advanced reservation management.
- [ ] Improved search and filtering capabilities.
- [ ] Additional data export options.
- [ ] Expanded application customization.
- [ ] Further interface and performance improvements.

## Contributing

Contributions, suggestions, and bug reports are welcome.

If you would like to contribute:

1. Fork the repository.
2. Create a feature branch.
3. Implement your changes.
4. Test your modifications.
5. Submit a pull request.

Please keep the code organized and follow the existing project structure.

## Auteur
 * **Johanes-mg** 

## License

A license has not yet been specified for this project.

Please contact the project owner for information about usage, modification, and distribution permissions.

---

<div align="center">

**JHotel Making Hotel Management Simpler.**

*Built with Rust. Designed for Efficiency.*

</div>

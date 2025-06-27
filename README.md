# FDTDM 1D Electromagnetic Wave Simulator

This is a terminal-based 1D FDTD (Finite-Difference Time-Domain) electromagnetic wave simulation, with optional real-time rendering and multi-threaded execution.

---

## ⚡ Performance Notes

By default, this application runs at **60 FPS** with a simulation size of **`N = 100,000`**.

At this size, the **single-threaded mode performs approximately twice as fast** as the multi-threaded mode, due to the overhead of managing threads and synchronization.  
However, when `N` is increased to **larger values (e.g., `1,000,000`)**, the **multi-threaded mode significantly outperforms** the single-threaded version, as the workload becomes large enough to justify parallel execution.

> 🧠 The optimal performance configuration depends on your system's CPU architecture, core count, and cache performance.

---

## 🖥 Terminal Usage

If the application is launched by double-clicking the `.exe`, it will open in your terminal's default location.

For best visual clarity and control, it's recommended to:
1. **Open a terminal manually.**
2. **Resize it to your preferred dimensions.**
3. **Run the application with command-line arguments.**

> ⚠️ **Do not resize the terminal window while the simulation is running**, as this may cause graphical artifacts or instability.

---

## 🛠 Command-Line Options

Run with **multi-threading enabled**:
```pwsh
fdtdm_1.exe --m
```

Run with **graphics disabled**:
```pwsh
fdtdm_1.exe --bench
```

Run with **both**:
```powershell
fdtdm_1.exe --m --bench
```

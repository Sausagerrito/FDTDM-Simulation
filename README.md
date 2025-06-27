By default this app runs at 60fps, with N = 100,000.

If ran from the .exe it will open into your terminals default location. It is better to open the terminal and scale it before you run the app. Running the app from the terminal also allows you to pass it arguments.

Do NOT resize the terminal window while the simulation is running.

To run multithreaded (This is slower on most CPUs):
```pwsh
fdtdm_1.exe --m
```

To run without graphics for accurate simulation timing:
```pwsh
fdtdm_1.exe --bench
```

Both:
```pwsh
fdtdm_1.exe --m --bench
```

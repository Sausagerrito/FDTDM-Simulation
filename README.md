By default this app runs at 60fps, with N = 100,000. At this N size, single threaded performance is about twice as fast as multithreaded, due to multi-threading overhead. HJowever at large N values such as 1,000,000 multi-threading significantly outpaces single-threading. The tipping point will depend on your computers performance.

If ran from the .exe it will open into your terminals default location. It is better to open the terminal and scale it before you run the app. Running the app from the terminal also allows you to pass it arguments.

Do NOT resize the terminal window while the simulation is running.

To run multithreaded:
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

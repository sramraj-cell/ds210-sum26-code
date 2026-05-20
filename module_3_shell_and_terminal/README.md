# Example Shell Commands Scripts!
 
Two example scripts:
1. `cat.sh` downloads a random cat photo
2. `nasa.sh` downloads a NASA astronomy photo of the day

To run, open your terminal, navigate to this directory and execute this command:
```bash
./cat.sh
```

Try to run the second script:
```bash
./nasa.sh
```

You will see a `Permission denied` error, because we did not make the nasa.sh script file executable!

```bash
chmod +x nasa.sh
./nasa.sh
```

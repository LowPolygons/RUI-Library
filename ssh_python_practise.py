import paramiko

hostname = "ui2.scarf.rl.ac.uk"
username = "djc23551"
password = input("Password: ")

client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

# Connect using a password
client.connect(hostname, username=username, password=password)

# Submit a Slurm job
stdin, stdout, stderr = client.exec_command("j")
stdin, stdout, stderr = client.exec_command("ls")

pwd_example = stdout.read().decode().strip()
print(f"LS OUTPUT: {pwd_example}")

client.close()

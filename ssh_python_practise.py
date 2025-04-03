import paramiko

hostname = "ui2.scarf.rl.ac.uk"
username = "djc23551"
password = "sm@5kd+nv8"

client = paramiko.SSHClient()
client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

# Connect using a password
client.connect(hostname, username=username, password=password)

# Submit a Slurm job
stdin, stdout, stderr = client.exec_command("pwd")
pwd_example = stdout.read().decode().strip()
print(f"PWD OUTPUT: {pwd_example}")

client.close()

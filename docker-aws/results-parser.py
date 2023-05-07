import csv
import os
import subprocess


BUCKET_NAME = "zk-benchmarking"


def main():
    subprocess.run([
        "aws", "s3", "cp", "s3://" + BUCKET_NAME "/",
        "results/", "--recursive"
    ])
    with open("output.csv", 'w', newline='') as write_file:
        writer = csv.writer(write_file)
        writer.writerow([
            "Instance type + job name + job size", "Proof duration",
            "Verify duration", "Output bytes", "Proof bytes"
        ])
        for filename in os.listdir(os.path.join(os.getcwd(), "results")):   
            with open("results/" + filename, 'r') as read_file:
                lines = read_file.readlines()

                i = 0
                while i < len(lines):
                    if lines[i].startswith("+ job_name"):
                         writer.writerow(getCsvRow(filename[:-4], i, lines))
                    i += 1


def getCsvRow(instance_type, index, file_lines):
    job_name = file_lines[index].split(':')[1].strip().strip('"')
    job_size = file_lines[index + 1].split(':')[1].strip()
    proof_duration = file_lines[index + 2].split(':')[1].strip()
    verify_duration = file_lines[index + 3].split(':')[1].strip()
    output_bytes = file_lines[index + 4].split(':')[1].strip()
    proof_bytes = file_lines[index + 5].split(':')[1].strip()
    first_col = "{} {} {}".format(instance_type, job_name, job_size)
    return [first_col, proof_duration, verify_duration, output_bytes, proof_bytes]


if __name__ == "__main__":
    main()

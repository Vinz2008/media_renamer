import os, shutil

folder_path = "test_files"

shutil.rmtree(folder_path)

os.mkdir(folder_path)

for i in range(10,20):
    temp_file_path = folder_path + "/series - " + str(i) + " - [BDrip] - (2022)"
    f = open(temp_file_path, "w")
    f.close()
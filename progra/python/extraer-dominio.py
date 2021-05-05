
import re

# sin regex
def domain_name(url):
    return url.split("//")[-1].split("www.")[-1].split(".")[0]

#con regex
def domain_name_re(url):
    return re.search('(https?://)?(www\d?\.)?(?P<name>[\w-]+)\.', url).group('name')

def main():
    url = input("ingrese url: ")
    print(domain_name_re(url))

main()

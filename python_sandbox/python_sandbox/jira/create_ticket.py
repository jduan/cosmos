import requests
import os


def main():

    jira_password = os.getenv('JIRA_PASSWORD')

    if not jira_password:
        print('Please set up your password via JIRA_PASSWORD')
        exit(1)

    # Initiate session with requests.
    s = requests.Session()

    response = s.get(url='https://jirarest.airbnb.biz/rest/api/latest/issue/BLDINFRA-542',
                     auth=('jingjing_duan', jira_password))

    print(response)


if __name__ == '__main__':
    main()


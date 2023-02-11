import pprint
import yaml

yaml_data = """
    id_party_details:
      pk: id
      domain: identity
      soundex: >-
        suffix_name,title_name,first_name,last_name,middle_name
      hashed: >-
        street_address_line1,city,state,phone_number,identification_number,ssn_last4
      hashed_with_salts:
        last_name_hashed:
          source_column_name: last_name
          salt: search_identity_pii
          length: -1
        last_name4_hashed:
          source_column_name: last_name
          salt: search_identity4_pii
          length: 4
"""

y = yaml.safe_load(yaml_data)
pprint.pprint(y)

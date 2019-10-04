import hydra


# strict mode is on by default
# You can access keys that don't exist. You can't add keys that aren't in the config file.
@hydra.main(config_path='conf', strict=True)
def my_app(cfg):
    print("Hydra config:")
    print(cfg.pretty())
    print()
    # print("driver: %s" % cfg.db.driver)
    # print("pass: %s" % cfg.db.password)


# You can also pass config via the command line: (they override the config file)
# python hydra_example.py db.driver=mysql db.user=omry db.pass=secret
#
# When using "config groups", pass the group name:
# python hydra_example.py db=postgres
if __name__ == '__main__':
    my_app()

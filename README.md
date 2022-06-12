# aws-mfa

Automation for temporary MFA credentials

USAGE:

```
    aws-mfa --token <TOKEN>
```

OPTIONS:

```
    -h, --help             Print help information
    -t, --token <TOKEN>    MFA token code (6 digits)
    -V, --version          Print version information
```


Prepare ```.mfaserial``` file in the same directory to specify the ARN of the mfa device.

```sh
$ cat .mfaserial
arn:aws:iam::123456789012:mfa/username
```
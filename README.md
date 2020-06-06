# sendle

This is cli tool to send pdf files to your kindle.

```sh
$ sendle send linux.pdf
```

## Installation

TODO: publish this.

## How to use

### Setup

Sendle uses gmail smtp server to send email to your kindle.  
You need to set credentials for gmail smtp server and kindle info you will send pdf files.

| Key                         | Default | Description |
|-----------------------------|---------|-------------|
| gmail_address               | -       | Your gmail address that will be used send email to kindles. |
| google_application_password | -       | Application password to certify smtp server.<br>Please read below how to get your application password. <br> [Google App Passwords](https://support.google.com/mail/answer/185833?hl=en) |
| kindle_name                 | default | Kindle name to be managed by Sendle. <br> Any Value is Ok. |
| kindle_mail_address         | -       | Send-to-Kindle-email address. <br> Please read below, how to get the mail address. <br> [Send to Kindle by E-mail](https://www.amazon.com/gp/sendtokindle/email) |

```sh
$ sendle config
gmail_address: example@gmail.com
google_application_password: expfldodhykovwjf
kindle_name: paper_white
kindle_mail_address: example_K4HblsQ4@gmail.com
```

### Send pdf files

```sh
$ sendle send linux.pdf
```

## Develop

### Unit Test
You must use nightly build to run unit tests.

Sendle uses [CodeSandwich/Mocktopus](https://github.com/CodeSandwich/Mocktopus) as mocking libray and it depends on nightly build because of using `#![feature]`.

```sh
$ rustup run nightly cargo t
```

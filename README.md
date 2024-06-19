# Pie

## Local Setup

### Ngrok

To check if Slack event subscription works well, you have to respond the challenge request described [url_verification](https://api.slack.com/events/url_verification). To do that, Ngrok is required to register an URL from local laptop.

```bash
brew install ngrok/ngrok/ngrok
ngrok config add-authtoken <token>

# This will create admin web interface to check the public URL to register in Slack API
ngrok http 8080
```

### Slack Apps

Go to `https://api.slack.com/apps` and navigate to the app you want to use. Go to `Event Subscriptions` and add the public URL from Ngrok.

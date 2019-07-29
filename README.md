# Rusty Reminder
Simple program that sends email reminders sourced from an Excel file.
- config.ini and reminders.xlsx must exist in same folder as program
- Excel file must have 'Sheet1' with 2 columns, date & message
- Only tested with Office365 smtp settings
- Reminder emails are sent to the same email address as the SMTP account
- When ran, program checks for any rows in the Excel file where the date equals today's date and sends a reminder email containing the contents of the message column.

## Config
config.ini example

    [smtp]  
    host=smtp.office365.com
    port=587  
    username=myuser  
    password=mypass  
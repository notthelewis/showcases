# Notes

When adding new message types, ensure to create a new file first,  following the conventions in the door message.
Then, in `./index.ts`, import the message type and export it in the `export const` block alongside header and door.
Finally, ensure to add an entry in `message_codes` for the new message type.

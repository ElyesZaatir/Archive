from pyOutlook import OutlookAccount
from pyOutlook.core.message import Message
from pyOutlook.core.contact import Contact
account = OutlookAccount('token 1')

message = Message(account, 'A body', 'A subject', [Contact('')])
message.attach(bytes('some bytes', 'utf-8'), 'bytes.txt')
message.send()

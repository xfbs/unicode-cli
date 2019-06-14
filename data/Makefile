UNICODE_VERSION = 12.1.0
UNICODE_BASE    = ftp://ftp.unicode.org/Public
UNICODE_FOLDER	= ucd
UNICODE_FILES	= UnicodeData.txt Blocks.txt

all: $(UNICODE_FILES)

%.txt:
	wget "$(UNICODE_BASE)/$(UNICODE_VERSION)/$(UNICODE_FOLDER)/$@"

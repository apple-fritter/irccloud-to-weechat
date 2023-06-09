# irccloud-to-weechat
This program demonstrates how Rust can be used to work with files and archives, parse data, and write data to files. In this implementation, we use Rust to convert an `IRCCloud` log export to `Weechat` log format.

I have also formed a program to convert IRCCloud logs to the [X-Chat](https://github.com/apple-fritter/logconvert.irccloud-to-xchat) format.

## How it works
This Rust program reads the contents of a zip file called `irccloud_logs.zip` and extracts text files with the `.txt` extension. It then processes the contents of each file, parsing the lines and extracting relevant information. The program then creates a new file in the same directory as the original file, with a ".weechatlog" extension, containing the parsed information.

The program uses several Rust standard library modules, including `std::fs`, `std::io`, and `std::path`, as well as the `zip` crate for working with zip files.

The `main()` function opens the zip file using `File::open()` and creates a `ZipArchive` object from the opened file. It then iterates over the files in the archive using a `for` loop and the `archive.len()` method.

For each file, the program uses the `by_index()` method of the ZipArchive object to obtain a file handle, which is then used to create a `BufReader` to read the contents of the file. The program then creates a new file with a name that matches the original file, but with a `.weechatlog` extension, using the File::create() method.

The program then iterates over the lines of the file using the `lines()` method of the BufReader. For each line, the program splits the line into three parts using the `splitn()` method of the str type. If the line does not contain enough information, the program skips it. Otherwise, the program extracts the `timestamp` and `message` from the line and formats them into a string. The formatted message is then written to the output file using the `writeln!()` macro.

## Concerns
1. Error handling: While the program does use unwrap() in several places to simplify the code, this can cause the program to panic if an error occurs. It might be better to use Result and match statements to handle errors in a more controlled way.
2. Performance: Depending on the size and number of files in the zip archive, this program might be slow or use a lot of memory. It might be worth optimizing the program to handle larger archives or implementing some kind of parallel processing.
3. File path handling: The program assumes that the input zip file is located in the current directory and that the output files should be written to the same directory. This might not always be the case, and the program might fail if it encounters unexpected file paths.
4. File extension handling: The program only looks for files with the ".txt" extension and skips any other files in the archive. This might not be sufficient if there are other kinds of files in the archive that need to be processed.
5. Encoding handling: The program assumes that the input files are encoded in UTF-8, which might not always be the case. It might be worth adding support for different encodings or detecting the encoding of the input files automatically.

## IRC Meta

### WeeChat
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. (Python)
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Record misspelled words in a TSV (tab-separated values) file. (Python)
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. (Python)
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Deprecated. Extract video information from a YouTube URL and post it back to the channel. (Python)
- [weechat.youtube-api](https://github.com/apple-fritter/weechat.youtube-api): Extract video information from a YouTube URL and post it back to the channel. (Python)

### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from IRCcloud format to Weechat format. (Rust)
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from IRCcloud format to XChat format. (Rust)

### X-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. (Python)
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. (Python bundle)

### Other
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format definition. (Rust)
- [scrimshaw](https://github.com/apple-fritter/scrimshaw): Create a quoteslist of any given user, from your driftwood formatted logs. (Rust)

### IRC usage considerations
When working with any project involving IRC (Internet Relay Chat), it's important to keep the following considerations in mind to ensure a positive and respectful environment for all participants.

#### Philosophy of Use
Tailor your project's behavior and responses to align with the expected norms and conventions of IRC. Take into account the preferences and expectations of IRC users, ensuring that your project provides a seamless and familiar experience within the IRC ecosystem.

#### Foster a Positive and Inclusive Environment
Respect and adhere to the guidelines and policies of the IRC platform you are using. Familiarize yourself with the platform's rules regarding script usage, automation, and acceptable behavior. Comply with the platform's Terms of Service, and be mindful of any limitations or restrictions imposed by the platform. Strive to create an inclusive and welcoming environment where all users can engage respectfully and comfortably.

#### Respect the Rights and Dignity of Other Users
Maintain a polite and courteous demeanor in all interactions. Uphold the fundamental principles of respect, avoiding engagement in illegal, inappropriate, or offensive behavior. This includes refraining from using derogatory or inflammatory language, sharing explicit, triggering, or offensive content, engaging in harassment, or launching personal attacks. Obtain explicit consent before interacting with other users or sending automated responses. Respect the privacy of other users and avoid invading their personal space without their permission.

#### Respect the IRC Community and Channels
Avoid disrupting the normal flow of conversation within IRC channels. Ensure that your project's actions and responses do not cause unnecessary disruptions or inconvenience to other users. Implement mechanisms to prevent spamming or flooding the channel with excessive or irrelevant messages. Handle errors gracefully, preventing unintended behavior or disruptions to the IRC platform or the experiences of other users.

#### Ensure Compatibility
Consider the potential variations in behavior across different IRC platforms and clients. While aiming for compatibility, be aware that certain functionalities may not be available or consistent across all platforms. Test your project on multiple IRC platforms and clients to ensure compatibility and provide the best possible experience for users.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License

These files released under the [MIT License](LICENSE).

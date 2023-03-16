QuoteUploader is a C# program that lets you bulk upload quotes to a Mongo database.

Simply move your txt file containing all your quotes inside the quotes_scrapped (default folder when you scrape) folder.
Run the program and when asked, write the name of the txt file so it can choose the file correctly.
The program will take care of parsing of the data so don't worry too much about that. The only thing is just take out the table of contents, page numbers, titles, and sub titles.
As we can see with out demo quote.txt, if takes care of all empty lines and tabbed lines.
Also, we take extra information such as how many words, letters and special character and upload thosse as well to the database.

The program is missing a file which is Secret.cs.
This file is the equivalent of an .env file.
You would just need to insert the data that matches your database information.
Replace all your information with your database below in the public static string and place it inside the Secret.cs.


    // The secret URI that let's you connect to your Mongo Cluster.
    public static readonly string ATLAS_URI = <ATLAS SECRET URI>;
    // The name of the Database.
    public static readonly string DATABASE_NAME = <Database Name>;
    // The name of the Collection.
    public static readonly string DATABASE_COLLECTION = <Database Collection>;

Before running the program make sure to put all your txt files in the quotes_scrapped (default folder when you scrape) folder.
To run the program, go inside the QuoteUploader directory and simply type "dotnet run" in your terminal.
The program will prompt you to type the name of the txt file, like mention above.
The program will parse the content and upload that parse data to the database.
Sit back and relax while it parses and sends to the database.
Enjoy!!!
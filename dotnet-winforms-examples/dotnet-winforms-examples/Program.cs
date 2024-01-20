using System;
using System.Diagnostics;
using System.IO;
using System.Threading;
using System.Windows.Forms;
using System.Text;

namespace dotnet_winforms_examples
{
    static class Program
    {
        static void CreateTEstFile(string path)
        {
            string content =
                @"
				<html>
				    <head>
					</head>
					<body>
					        <input type='text' id='inputText' />
                            <button onclick='alert(1)'>Click me</button>
							<div id='fillMe'>
								Original text
							</div>
					</body>
				</html>
			";
            try
            {
                // Create the file, or overwrite if the file exists.
                using (FileStream fs = File.Create(path))
                {
                    byte[] info = new UTF8Encoding(true).GetBytes(content);
                    // Add some information to the file.
                    fs.Write(info, 0, info.Length);
                }

                // Open the stream and read it back.
                using (StreamReader sr = File.OpenText(path))
                {
                    string s = "";
                    while ((s = sr.ReadLine()) != null)
                    {
                        Console.WriteLine(s);
                    }
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.ToString());
            }
        }

        [STAThread]
        static void Main()
        {
            string path = Directory.GetCurrentDirectory();
            string tmpHtml = Path.Join(path, "tmp.html");
            CreateTEstFile(tmpHtml);
            if (!File.Exists(tmpHtml))
            {
                Console.WriteLine("tmp.html at path '{0}' is missing", tmpHtml);
                return;
            }
            string tmpPath = "file://" + tmpHtml;
            Application.SetHighDpiMode(HighDpiMode.SystemAware);
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Thread t1 = new Thread(() =>
            {
                Application.Run(new Form1());
            });
            Thread t2 = new Thread(() =>
            {
                Application.Run(new Form1());
            });
            Thread t3 = new Thread(() =>
            {
                Application.Run(new FormWindows(tmpPath));
            });
            t3.SetApartmentState(ApartmentState.STA);
            t1.Start();
            t2.Start();
            t3.Start();
            t1.Join();
            t2.Join();
            t3.Join();
        }
    }
}

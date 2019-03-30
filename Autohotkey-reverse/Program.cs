using System;
using System.Text;
using System.IO;
using System.Xml;
using System.Reflection;

namespace Ahkr
{
    class Program
    { 
        private static FolderExists Folder { get; set; } = new FolderExists();

        private static string GetResourceTextFile(string filename)
        {
            string result = string.Empty;

            using (Stream stream = Assembly.GetExecutingAssembly().GetManifestResourceStream("Ahkr." + filename))
            {
                using (StreamReader sr = new StreamReader(stream))
                {
                    result = sr.ReadToEnd();
                }
            }
            return result;
        }
        private static byte[] HexStringToBytes(string hexString)
        {
            var bytes = new byte[hexString.Length / 2];
            for (int i = 0; i < bytes.Length; i++)
            {
                string currentHex = hexString.Substring(i * 2, 2);
                bytes[i] = Convert.ToByte(currentHex, 16);
            }
            return bytes;
        }
        private static string[] append(string[] array, string value)
        {
            Array.Resize(ref array, array.Length + 1);
            array[array.Length - 1] = value;
            return array;
        }
        private static string[] removeLast(string[] array)
        {
            Array.Resize(ref array, array.Length - 1);
            return array;
        }
        private static string[] ripple(string[] data, string new_value)
        {
            string[] output = new string[data.Length];
            for (int i = 0; i < data.Length-1; i++)
            {
                output[i] = data[i + 1];
            }
            output[data.Length-1] = new_value;
            return output;
        }
        private static void ConvertScript(string[] args)
        {
            string exepath = "";
            string exe = "";
            string outpath = "";
            string outx = "";
            string[] s = args[0].Replace('/', '\\').Split('\\');
            string[] sc = args[1].Replace('/', '\\').Split('\\');
            if (!args[0].Replace("/", "\\").Split('\\')[args[0].Replace("/", "\\").Split('\\').Length - 1].Contains(".exe"))
            {
                Console.WriteLine("Autohot Decomp: That isn't an EXE");
                return;
            }
            foreach (string x in s)
            {
                if (!x.Contains(".exe"))
                {
                    exepath += x + "\\";
                }
                else
                {
                    exe = x;
                }
            }
            foreach (string x in sc)
            {
                if (!x.Contains(".ahk"))
                {
                    outpath += x + "\\";
                }
                else
                {
                    outx = x;
                }
            }
            exepath = Folder.NewPath(exepath.Replace("\\\\", "\\"));
            outpath = Folder.NewPath(outpath.Replace("\\\\", "\\"));
            outpath += outx;
            if (!Folder.DoesExist(args[1]))
            {
                Console.WriteLine("Autohot Decomp: The output folder is unaccessible");
            }
            if (Folder.DoesExist(exepath))
            {
                if (File.Exists(exepath + exe))
                {
                    string path = exepath + exe;
                    string[] header = new string[128];
                    using (FileStream file = new FileStream(path, FileMode.Open))
                    {
                        string[] byteSearch = new string[12];
                        string[] byteSearch2 = new string[6];
                        for (int i = 0; i < byteSearch.Length; i++)
                        {
                            byteSearch[i] = "";
                        }
                        for (int i = 0; i < byteSearch2.Length; i++)
                        {
                            byteSearch2[i] = "";
                        }
                        int entp = 0;
                        bool searching = true;
                        bool cVersionCheck = true;
                        bool cScriptContent = true;
                        string cVersion = "";
                        string code = "";
                        for (int i = 0; i < file.Length - 129; i++)
                        {
                            if (searching)
                            {
                                string into = file.ReadByte().ToString("X");
                                if (into.Length == 1)
                                {
                                    into = "0" + into;
                                }
                                byteSearch = ripple(byteSearch, into);
                                if (byteSearch[0] == "3C")
                                {
                                    string entry = "";
                                    foreach (string x in byteSearch)
                                    {
                                        if (entry.Length == 1)
                                        {
                                            entry += "0" + x;
                                        }
                                        else
                                        {
                                            entry += x;
                                        }
                                    }
                                    if (entry == "3C434F4D50494C45523A2076")
                                    {
                                        entp = i;
                                        searching = false;
                                    }
                                }
                            }
                            else if (cVersionCheck)
                            {
                                string xInput = file.ReadByte().ToString("X");
                                if (xInput == "3E")
                                {
                                    cVersionCheck = false;
                                }
                                else
                                {
                                    cVersion += Encoding.UTF8.GetString(HexStringToBytes(xInput));
                                }
                            }
                            else if (cScriptContent)
                            {
                                string into = file.ReadByte().ToString("X");
                                if (into.Length == 1)
                                {
                                    into = "0" + into;
                                }
                                byteSearch2 = ripple(byteSearch2, into);
                                code += byteSearch2[byteSearch2.Length - 1];
                                if (byteSearch2[0] == "0A" || byteSearch2[0] == "00")
                                {
                                    string entry = "";
                                    foreach (string x in byteSearch2)
                                    {
                                        if (entry.Length == 1)
                                        {
                                            entry += "0" + x;
                                        }
                                        else
                                        {
                                            entry += x;
                                        }
                                    }
                                    if (entry == "0A5041440000" || entry == "000001000500")
                                    {
                                        code = code.Replace("0A5041440000", "").Replace("000001000500", "");
                                        code = code.Remove(code.IndexOf("0A"), ("0A").Length);
                                        code = Encoding.UTF8.GetString(HexStringToBytes(code));
                                        cScriptContent = false;
                                    }
                                }

                            }
                        }
                        Console.WriteLine("The script was compiled with Autohotkey version " + cVersion);
                        if (File.Exists(outpath))
                        {
                            File.Delete(outpath);
                        }
                        File.WriteAllText(outpath, code);
                    }

                }
                else
                {
                    Console.WriteLine("Autohot Decomp: The file doesn't exist");
                }
            }
            else
            {
                Console.WriteLine("Autohot Decomp: Folder doesn't exist");
            }
        }
        static void Main(string[] args)
        {
            if (args.Length == 2)
            {
                ConvertScript(args);
            }
            else if (args.Length == 1)
            {
                string exepath = "";
                string exe = "";
                string[] s = args[0].Replace('/', '\\').Split('\\');
                if (!args[0].Replace("/", "\\").Split('\\')[args[0].Replace("/", "\\").Split('\\').Length - 1].Contains(".exe"))
                {
                    Console.WriteLine("Autohot Decomp: That isn't an EXE");
                    return;
                }
                foreach (string x in s)
                {
                    if (!x.Contains(".exe"))
                    {
                        exepath += x + "\\";
                    }
                    else
                    {
                        exe = x;
                    }
                }
                exepath = exepath.Replace("\\\\", "\\");
                string path = exepath + exe;

                if (Folder.DoesExist(exepath))
                {
                    if (File.Exists(path))
                    {
                        if (!File.Exists(path.Replace(".exe", ".ahk")))
                        {
                            string[] ex = new string[2];
                            ex[0] = args[0];
                            ex[1] = path.Replace(".exe", ".ahk");
                            ConvertScript(ex);
                        } else
                        {
                            Console.WriteLine("Autohot Decomp: The autohotkey file already exists!");
                        }
                    } else
                    {
                        Console.WriteLine("Autohot Decomp: The exe doesn't exist");
                    }
                }
                else
                {
                    Console.WriteLine("Autohot Decomp: Folder doesn't exist");
                }
            } else
            {
                Console.WriteLine("Autohot Decomp: That is too many arguments");
            }
        }
    }
    public class FolderExists
    {
        private string backtrackDetect(string str)
        {
            string[] parts = str.Split('\\');
            string[] list = new string[0];
            string output = "";
            if (str.Contains(".."))
            {
                foreach (string part in parts)
                {
                    if (part != "..")
                    {
                        list = append(list, part);
                    }
                    else
                    {
                        list = removeLast(list);
                    }
                }
                foreach (string part in list)
                {
                    output += part;
                    output += "\\";
                    if (output.Contains("\\\\"))
                    {
                        output = output.Substring(0, output.LastIndexOf("\\\\") + 1);
                    }
                }
            }
            else
            {
                output = str;
            }
            return output;
        }
        private string[] append(string[] array, string value)
        {
            Array.Resize(ref array, array.Length + 1);
            array[array.Length - 1] = value;
            return array;
        }
        private string[] removeLast(string[] array)
        {
            Array.Resize(ref array, array.Length - 1);
            return array;
        }
        private string detectStarting(string str)
        {
            string output = Environment.CurrentDirectory;
            str = str.Replace("/", "\\");
            if (str.StartsWith("..\\"))
            {
                int found = Environment.CurrentDirectory.LastIndexOf("\\");
                int rm = str.IndexOf("\\");
                str = str.Substring(rm, str.Length - rm);
                output = Environment.CurrentDirectory.Substring(0, found);
                output += str;
            }
            else if (str.StartsWith(".\\") || str.StartsWith("\\"))
            {
                int rm = str.IndexOf("\\");
                str = str.Substring(rm, str.Length - rm);
                output = Environment.CurrentDirectory;
                output += str;
            }
            else if (Directory.Exists(str))
            {
                output = str;
            }
            else if (!Directory.Exists(str) && str != "")
            {
                output = "Folder doesn't exist";
            }
            output = output.Replace("/", "\\");
            return output;
        }
        public bool DoesExist(string args)
        {
            bool exists = false;
            string folder;
            folder = detectStarting(args);
            folder = backtrackDetect(folder);
            if (folder != "Folder doesn't exist")
            {
                exists = true;
            }
            return exists;
        }
        public string NewPath(string str)
        {
            string output = "";
            string folder;
            folder = detectStarting(str);
            output = backtrackDetect(folder);
            return output;
        }
    }
}

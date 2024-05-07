export interface IOptions {
  /**
   * The path to the HTML file that will be used as a template.
   * eg:
   *  path.resolve(__dirname, 'index.html')
   */
  template: string;
  /**
   * An object containing the variables that will be injected into the HTML file.
   * eg: 
   *  data {
   *    title: 'My App',
   *  }
   */
  data: Record<string,string>
}
const binPath: (options?:IOptions)=>[string, typeof options];
export default binPath;

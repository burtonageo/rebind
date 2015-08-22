[rebind](https://github.com/burtonageo/rebind)
==============================================

A library for binding input keys to actions, and modifying mouse behaviour. Keys can be
bound to actions, and then translated during runtime. `Keys` are mapped to `Actions` using
a `HashMap`, so lookup time is constant. 

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
 
Main improvements to be made (in order from most important to least important):
-------------------------------------------------------------------------------
 
* Implement conversion from InputTranslator to InputRebind
* Improve the API: Is the distinction between InputRebind and InputTranslator necessary or useful?
* Add serialisation
* Add mouse sensitivity options
* Add tests/benchmarks

Contributions welcome.

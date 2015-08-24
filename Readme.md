[rebind](https://github.com/burtonageo/rebind)
==============================================

A library for binding input keys to actions, and modifying mouse behaviour. Keys can be
bound to actions, and then translated during runtime. `Keys` are mapped to `Actions` using
a `HashMap`, so lookup time is constant.

Copyright (c) 2015 Rebind developers.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

Main improvements to be made:
-----------------------------

* Implement conversion from InputTranslator to InputRebind
* Improve the API: Is the distinction between InputRebind and InputTranslator necessary or useful?
* Add serialisation
* Add mouse sensitivity options
* Add tests/benchmarks

Contributions welcome.

(function(e){function n(n){for(var o,i,u=n[0],a=n[1],l=n[2],f=0,p=[];f<u.length;f++)i=u[f],Object.prototype.hasOwnProperty.call(r,i)&&r[i]&&p.push(r[i][0]),r[i]=0;for(o in a)Object.prototype.hasOwnProperty.call(a,o)&&(e[o]=a[o]);s&&s(n);while(p.length)p.shift()();return c.push.apply(c,l||[]),t()}function t(){for(var e,n=0;n<c.length;n++){for(var t=c[n],o=!0,u=1;u<t.length;u++){var a=t[u];0!==r[a]&&(o=!1)}o&&(c.splice(n--,1),e=i(i.s=t[0]))}return e}var o={},r={app:0},c=[];function i(n){if(o[n])return o[n].exports;var t=o[n]={i:n,l:!1,exports:{}};return e[n].call(t.exports,t,t.exports,i),t.l=!0,t.exports}i.m=e,i.c=o,i.d=function(e,n,t){i.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},i.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},i.t=function(e,n){if(1&n&&(e=i(e)),8&n)return e;if(4&n&&"object"===typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(i.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var o in e)i.d(t,o,function(n){return e[n]}.bind(null,o));return t},i.n=function(e){var n=e&&e.__esModule?function(){return e["default"]}:function(){return e};return i.d(n,"a",n),n},i.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},i.p="/ornithology-pi/";var u=window["webpackJsonp"]=window["webpackJsonp"]||[],a=u.push.bind(u);u.push=n,u=u.slice();for(var l=0;l<u.length;l++)n(u[l]);var s=a;c.push([0,"chunk-vendors"]),t()})({0:function(e,n,t){e.exports=t("56d7")},"3ed5":function(e,n,t){"use strict";t("d7a8")},"56d7":function(e,n,t){"use strict";t.r(n);t("e260"),t("e6cf"),t("cca6"),t("a79d");var o=t("7a23"),r={id:"app"};function c(e,n,t,c,i,u){var a=Object(o["f"])("BLEDeviceList");return Object(o["e"])(),Object(o["b"])("div",r,[Object(o["d"])(a)])}t("ac1f"),t("841c");var i=Object(o["c"])("h1",null,"Bluetooth Devices",-1);function u(e,n,t,r,c,u){return Object(o["e"])(),Object(o["b"])("div",null,[i,Object(o["c"])("button",{onClick:n[0]||(n[0]=function(){return u.search&&u.search.apply(u,arguments)})},"Search")])}t("b0c0");var a={name:"BLEDeviceList",components:{},data:function(){return{services:[4660]}},methods:{search:function(){navigator.bluetooth.requestDevice({filters:[{name:"ornithology-pi"}]}).then((function(e){return console.log(e.name),console.log(e),e.gatt.connect()})).then((function(e){console.log(e)})).catch((function(e){console.error(e)}))},handleScannedDevices:function(e){console.log("Scanned devices:",e)},handleConnectedDevices:function(e){console.log("Connected devices:",e)}}},l=t("6b0d"),s=t.n(l);const f=s()(a,[["render",u]]);var p=f,d={name:"App",components:{BLEDeviceList:p}};t("3ed5");const v=s()(d,[["render",c]]);var b=v;Object(o["a"])(b).mount("#app")},d7a8:function(e,n,t){}});
//# sourceMappingURL=app.29c5c8e4.js.map
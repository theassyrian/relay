(self.webpackChunk=self.webpackChunk||[]).push([[84211],{3905:(t,e,n)=>{"use strict";n.d(e,{Zo:()=>u,kt:()=>d});var a=n(67294);function r(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function l(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(t);e&&(a=a.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,a)}return n}function o(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?l(Object(n),!0).forEach((function(e){r(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):l(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function i(t,e){if(null==t)return{};var n,a,r=function(t,e){if(null==t)return{};var n,a,r={},l=Object.keys(t);for(a=0;a<l.length;a++)n=l[a],e.indexOf(n)>=0||(r[n]=t[n]);return r}(t,e);if(Object.getOwnPropertySymbols){var l=Object.getOwnPropertySymbols(t);for(a=0;a<l.length;a++)n=l[a],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(r[n]=t[n])}return r}var p=a.createContext({}),m=function(t){var e=a.useContext(p),n=e;return t&&(n="function"==typeof t?t(e):o(o({},e),t)),n},u=function(t){var e=m(t.components);return a.createElement(p.Provider,{value:e},t.children)},s={inlineCode:"code",wrapper:function(t){var e=t.children;return a.createElement(a.Fragment,{},e)}},c=a.forwardRef((function(t,e){var n=t.components,r=t.mdxType,l=t.originalType,p=t.parentName,u=i(t,["components","mdxType","originalType","parentName"]),c=m(n),d=r,k=c["".concat(p,".").concat(d)]||c[d]||s[d]||l;return n?a.createElement(k,o(o({ref:e},u),{},{components:n})):a.createElement(k,o({ref:e},u))}));function d(t,e){var n=arguments,r=e&&e.mdxType;if("string"==typeof t||r){var l=n.length,o=new Array(l);o[0]=c;var i={};for(var p in e)hasOwnProperty.call(e,p)&&(i[p]=e[p]);i.originalType=t,i.mdxType="string"==typeof t?t:r,o[1]=i;for(var m=2;m<l;m++)o[m]=n[m];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}c.displayName="MDXCreateElement"},69516:(t,e,n)=>{"use strict";n.r(e),n.d(e,{frontMatter:()=>i,contentTitle:()=>p,metadata:()=>m,toc:()=>u,default:()=>c});var a=n(22122),r=n(19756),l=(n(67294),n(3905)),o=["components"],i={id:"compatibility-cheatsheet",title:"Compatibility Cheatsheet",original_id:"compatibility-cheatsheet"},p=void 0,m={unversionedId:"compatibility-cheatsheet",id:"version-v4.0.0/compatibility-cheatsheet",isDocsHomePage:!1,title:"Compatibility Cheatsheet",description:"What works with what? Relay Compat ('react-relay/compat') is the most flexible.",source:"@site/versioned_docs/version-v4.0.0/Modern-CompatibilityCheatsheet.md",sourceDirName:".",slug:"/compatibility-cheatsheet",permalink:"/docs/v4.0.0/compatibility-cheatsheet",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v4.0.0/Modern-CompatibilityCheatsheet.md",version:"v4.0.0",lastUpdatedBy:"Andrey Lunyov",lastUpdatedAt:1643042640,formattedLastUpdatedAt:"1/24/2022",frontMatter:{id:"compatibility-cheatsheet",title:"Compatibility Cheatsheet",original_id:"compatibility-cheatsheet"}},u=[{value:"Can RelayRootContainer use:",id:"can-relayrootcontainer-use",children:[]},{value:"Can QueryRenderer using Classic Environment (<code>Store</code> in <code>react-relay/classic</code>) use:",id:"can-queryrenderer-using-classic-environment-store-in-react-relayclassic-use",children:[]},{value:"Can QueryRenderer using Modern Environment use:",id:"can-queryrenderer-using-modern-environment-use",children:[]},{value:"Can React Modern Component use:",id:"can-react-modern-component-use",children:[]},{value:"Can React Compat Component use:",id:"can-react-compat-component-use",children:[]},{value:"Can React Classic Component use:",id:"can-react-classic-component-use",children:[]}],s={toc:u};function c(t){var e=t.components,n=(0,r.Z)(t,o);return(0,l.kt)("wrapper",(0,a.Z)({},s,n,{components:e,mdxType:"MDXLayout"}),(0,l.kt)("p",null,"What works with what? Relay Compat (",(0,l.kt)("inlineCode",{parentName:"p"},"'react-relay/compat'"),") is the most flexible.\nCompat components and mutations can be used by everything. Compat components can also have any kind of children."),(0,l.kt)("p",null,"However components using the Relay Modern API (",(0,l.kt)("inlineCode",{parentName:"p"},"'react-relay'"),") and the Relay Classic API (",(0,l.kt)("inlineCode",{parentName:"p"},"'react-relay/classic'"),") cannot be used with each other."),(0,l.kt)("h3",{id:"can-relayrootcontainer-use"},"Can RelayRootContainer use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No")))),(0,l.kt)("h3",{id:"can-queryrenderer-using-classic-environment-store-in-react-relayclassic-use"},"Can QueryRenderer using Classic Environment (",(0,l.kt)("inlineCode",{parentName:"h3"},"Store")," in ",(0,l.kt)("inlineCode",{parentName:"h3"},"react-relay/classic"),") use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No")))),(0,l.kt)("h3",{id:"can-queryrenderer-using-modern-environment-use"},"Can QueryRenderer using Modern Environment use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes")))),(0,l.kt)("h3",{id:"can-react-modern-component-use"},"Can React Modern Component use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes")))),(0,l.kt)("h3",{id:"can-react-compat-component-use"},"Can React Compat Component use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes","*"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes")))),(0,l.kt)("p",null,"*"," Modern API doesn't support mutation fragments. You might have to inline the mutation fragments from your legacy mutation in the fragment of the component."),(0,l.kt)("h3",{id:"can-react-classic-component-use"},"Can React Classic Component use:"),(0,l.kt)("table",null,(0,l.kt)("thead",{parentName:"table"},(0,l.kt)("tr",{parentName:"thead"},(0,l.kt)("th",{parentName:"tr",align:null},"Classic Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Component"),(0,l.kt)("th",{parentName:"tr",align:null},"Classic Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Compat Mutation"),(0,l.kt)("th",{parentName:"tr",align:null},"Modern Mutation"))),(0,l.kt)("tbody",{parentName:"table"},(0,l.kt)("tr",{parentName:"tbody"},(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"Yes"),(0,l.kt)("td",{parentName:"tr",align:null},"No")))))}c.isMDXComponent=!0}}]);
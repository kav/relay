"use strict";(self.webpackChunkrelay_website=self.webpackChunkrelay_website||[]).push([[17596],{3905:(e,r,t)=>{t.r(r),t.d(r,{MDXContext:()=>l,MDXProvider:()=>u,mdx:()=>g,useMDXComponents:()=>c,withMDXComponents:()=>p});var o=t(67294);function i(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function a(){return a=Object.assign||function(e){for(var r=1;r<arguments.length;r++){var t=arguments[r];for(var o in t)Object.prototype.hasOwnProperty.call(t,o)&&(e[o]=t[o])}return e},a.apply(this,arguments)}function n(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);r&&(o=o.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,o)}return t}function s(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?n(Object(t),!0).forEach((function(r){i(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):n(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function d(e,r){if(null==e)return{};var t,o,i=function(e,r){if(null==e)return{};var t,o,i={},a=Object.keys(e);for(o=0;o<a.length;o++)t=a[o],r.indexOf(t)>=0||(i[t]=e[t]);return i}(e,r);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(o=0;o<a.length;o++)t=a[o],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(i[t]=e[t])}return i}var l=o.createContext({}),p=function(e){return function(r){var t=c(r.components);return o.createElement(e,a({},r,{components:t}))}},c=function(e){var r=o.useContext(l),t=r;return e&&(t="function"==typeof e?e(r):s(s({},r),e)),t},u=function(e){var r=c(e.components);return o.createElement(l.Provider,{value:r},e.children)},m={inlineCode:"code",wrapper:function(e){var r=e.children;return o.createElement(o.Fragment,{},r)}},f=o.forwardRef((function(e,r){var t=e.components,i=e.mdxType,a=e.originalType,n=e.parentName,l=d(e,["components","mdxType","originalType","parentName"]),p=c(t),u=i,f=p["".concat(n,".").concat(u)]||p[u]||m[u]||a;return t?o.createElement(f,s(s({ref:r},l),{},{components:t})):o.createElement(f,s({ref:r},l))}));function g(e,r){var t=arguments,i=r&&r.mdxType;if("string"==typeof e||i){var a=t.length,n=new Array(a);n[0]=f;var s={};for(var d in r)hasOwnProperty.call(r,d)&&(s[d]=r[d]);s.originalType=e,s.mdxType="string"==typeof e?e:i,n[1]=s;for(var l=2;l<a;l++)n[l]=t[l];return o.createElement.apply(null,n)}return o.createElement.apply(null,t)}f.displayName="MDXCreateElement"},28224:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>c,contentTitle:()=>l,default:()=>f,frontMatter:()=>d,metadata:()=>p,toc:()=>u});var o=t(83117),i=t(80102),a=(t(67294),t(3905)),n=t(44996),s=["components"],d={id:"editor-support",title:"Editor Support",slug:"/editor-support/",keywords:["LSP","Language Server Protocol","VS Code","VSCode"]},l=void 0,p={unversionedId:"editor-support",id:"version-v14.1.0/editor-support",title:"Editor Support",description:"TL;DR: We have a VS Code Extension",source:"@site/versioned_docs/version-v14.1.0/editor-support.md",sourceDirName:".",slug:"/editor-support/",permalink:"/docs/editor-support/",draft:!1,editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v14.1.0/editor-support.md",tags:[],version:"v14.1.0",lastUpdatedBy:"Tianyu Yao",lastUpdatedAt:1659131717,formattedLastUpdatedAt:"7/29/2022",frontMatter:{id:"editor-support",title:"Editor Support",slug:"/editor-support/",keywords:["LSP","Language Server Protocol","VS Code","VSCode"]},sidebar:"docs",previous:{title:"Videos",permalink:"/docs/principles-and-architecture/videos/"},next:{title:"Community Learning Resources",permalink:"/docs/community-learning-resources/"}},c={},u=[{value:"Relay compiler errors surface as red squiggles directly in your editor",id:"relay-compiler-errors-surface-as-red-squiggles-directly-in-your-editor",level:4},{value:"Autocomplete throughout your GraphQL tagged template literals",id:"autocomplete-throughout-your-graphql-tagged-template-literals",level:4},{value:"Hover to see type information and documentation about Relay-specific features",id:"hover-to-see-type-information-and-documentation-about-relay-specific-features",level:4},{value:"<code>@deprecated</code> fields are rendered using <del>strikethrough</del>",id:"deprecated-fields-are-rendered-using-strikethrough",level:4},{value:"Click-to-definition for fragments, fields and types",id:"click-to-definition-for-fragments-fields-and-types",level:4},{value:"Quick fix suggestions for common errors",id:"quick-fix-suggestions-for-common-errors",level:4},{value:"Language Server",id:"language-server",level:2},{value:"Why Have a Relay-Specific Editor Extension?",id:"why-have-a-relay-specific-editor-extension",level:2}],m={toc:u};function f(e){var r=e.components,t=(0,i.Z)(e,s);return(0,a.mdx)("wrapper",(0,o.Z)({},m,t,{components:r,mdxType:"MDXLayout"}),(0,a.mdx)("p",null,(0,a.mdx)("em",{parentName:"p"},"TL;DR: We have a ",(0,a.mdx)("a",{parentName:"em",href:"https://marketplace.visualstudio.com/items?itemName=meta.relay"},"VS Code Extension"))),(0,a.mdx)("hr",null),(0,a.mdx)("p",null,"The Relay compiler has a rich understanding of the GraphQL embedded in your code. We want to use that understanding to imporve the developer experience of writing apps with Relay. So, starting in ",(0,a.mdx)("a",{parentName:"p",href:"https://github.com/facebook/relay/releases/tag/v14.0.0"},"v14.0.0"),", the new Rust Relay compiler can provide language features directly in your code editor. This means:"),(0,a.mdx)("h4",{id:"relay-compiler-errors-surface-as-red-squiggles-directly-in-your-editor"},"Relay compiler errors surface as red squiggles directly in your editor"),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/diagnostics.png")}),(0,a.mdx)("h4",{id:"autocomplete-throughout-your-graphql-tagged-template-literals"},"Autocomplete throughout your GraphQL tagged template literals"),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/autocomplete.png")}),(0,a.mdx)("h4",{id:"hover-to-see-type-information-and-documentation-about-relay-specific-features"},"Hover to see type information and documentation about Relay-specific features"),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/hover.png")}),(0,a.mdx)("h4",{id:"deprecated-fields-are-rendered-using-strikethrough"},(0,a.mdx)("inlineCode",{parentName:"h4"},"@deprecated")," fields are rendered using ",(0,a.mdx)("del",{parentName:"h4"},"strikethrough")),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/deprecated.png")}),(0,a.mdx)("h4",{id:"click-to-definition-for-fragments-fields-and-types"},"Click-to-definition for fragments, fields and types"),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/go-to-def.gif")}),(0,a.mdx)("h4",{id:"quick-fix-suggestions-for-common-errors"},"Quick fix suggestions for common errors"),(0,a.mdx)("img",{src:(0,n.default)("img/docs/editor-support/code-actions.png")}),(0,a.mdx)("h2",{id:"language-server"},"Language Server"),(0,a.mdx)("p",null,"The editor support is implemented using the ",(0,a.mdx)("a",{parentName:"p",href:"https://microsoft.github.io/language-server-protocol/"},"Language Server Protocol")," which means it can be used by a variety of editors, but in tandem with this release, ",(0,a.mdx)("a",{parentName:"p",href:"https://twitter.com/b_ez_man"},"Terence Bezman")," from ",(0,a.mdx)("a",{parentName:"p",href:"https://www.coinbase.com/"},"Coinbase")," has contributed an official VS Code extension."),(0,a.mdx)("p",null,(0,a.mdx)("a",{parentName:"p",href:"https://marketplace.visualstudio.com/items?itemName=meta.relay"},(0,a.mdx)("strong",{parentName:"a"},"Find it here!"))),(0,a.mdx)("h2",{id:"why-have-a-relay-specific-editor-extension"},"Why Have a Relay-Specific Editor Extension?"),(0,a.mdx)("p",null,"The GraphQL foundation has an official language server and ",(0,a.mdx)("a",{parentName:"p",href:"https://marketplace.visualstudio.com/items?itemName=GraphQL.vscode-graphql"},"VS Code extension")," which provides editor support for GraphQL generically. This can provide a good baseline experience, but for Relay users, getting this information directly from the Relay compiler offers a number of benefits:"),(0,a.mdx)("ul",null,(0,a.mdx)("li",{parentName:"ul"},"Relay compiler errors can surface directly in the editor as \u201cproblems\u201d, often with suggested quick fixes"),(0,a.mdx)("li",{parentName:"ul"},"Hover information is aware Relay-specific features and directives and can link out to relevant documentation")))}f.isMDXComponent=!0}}]);
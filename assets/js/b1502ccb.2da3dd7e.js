(self.webpackChunk=self.webpackChunk||[]).push([[31982],{3905:(e,t,r)=>{"use strict";r.r(t),r.d(t,{MDXContext:()=>s,MDXProvider:()=>m,mdx:()=>v,useMDXComponents:()=>d,withMDXComponents:()=>u});var n=r(67294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(){return i=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var r=arguments[t];for(var n in r)Object.prototype.hasOwnProperty.call(r,n)&&(e[n]=r[n])}return e},i.apply(this,arguments)}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function l(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function c(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var s=n.createContext({}),u=function(e){return function(t){var r=d(t.components);return n.createElement(e,i({},t,{components:r}))}},d=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):l(l({},t),e)),r},m=function(e){var t=d(e.components);return n.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,o=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),u=d(r),m=a,f=u["".concat(o,".").concat(m)]||u[m]||p[m]||i;return r?n.createElement(f,l(l({ref:t},s),{},{components:r})):n.createElement(f,l({ref:t},s))}));function v(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=f;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:a,o[1]=l;for(var s=2;s<i;s++)o[s]=r[s];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},36742:(e,t,r)=>{"use strict";r.r(t),r.d(t,{default:()=>p});var n=r(79973),a=r(67294),i=r(73727),o=r(52263),l=r(13919),c=r(10412),s=(0,a.createContext)({collectLink:function(){}}),u=r(44996),d=r(18780),m=["isNavLink","to","href","activeClassName","isActive","data-noBrokenLinkCheck","autoAddBaseUrl"];const p=function(e){var t,r,p=e.isNavLink,f=e.to,v=e.href,y=e.activeClassName,h=e.isActive,g=e["data-noBrokenLinkCheck"],b=e.autoAddBaseUrl,w=void 0===b||b,x=(0,n.Z)(e,m),k=(0,o.default)().siteConfig,O=k.trailingSlash,E=k.baseUrl,j=(0,u.useBaseUrlUtils)().withBaseUrl,C=(0,a.useContext)(s),N=f||v,R=(0,l.Z)(N),P=null==N?void 0:N.replace("pathname://",""),U=void 0!==P?(r=P,w&&function(e){return e.startsWith("/")}(r)?j(r):r):void 0;U&&R&&(U=(0,d.applyTrailingSlash)(U,{trailingSlash:O,baseUrl:E}));var D=(0,a.useRef)(!1),M=p?i.OL:i.rU,T=c.default.canUseIntersectionObserver,S=(0,a.useRef)();(0,a.useEffect)((function(){return!T&&R&&null!=U&&window.docusaurus.prefetch(U),function(){T&&S.current&&S.current.disconnect()}}),[S,U,T,R]);var _=null!==(t=null==U?void 0:U.startsWith("#"))&&void 0!==t&&t,B=!U||!R||_;return U&&R&&!_&&!g&&C.collectLink(U),B?a.createElement("a",Object.assign({href:U},N&&!R&&{target:"_blank",rel:"noopener noreferrer"},x)):a.createElement(M,Object.assign({},x,{onMouseEnter:function(){D.current||null==U||(window.docusaurus.preload(U),D.current=!0)},innerRef:function(e){var t,r;T&&e&&R&&(t=e,r=function(){null!=U&&window.docusaurus.prefetch(U)},S.current=new window.IntersectionObserver((function(e){e.forEach((function(e){t===e.target&&(e.isIntersecting||e.intersectionRatio>0)&&(S.current.unobserve(t),S.current.disconnect(),r())}))})),S.current.observe(t))},to:U||""},p&&{isActive:h,activeClassName:y}))}},13919:(e,t,r)=>{"use strict";function n(e){return!0===/^(\w*:|\/\/)/.test(e)}function a(e){return void 0!==e&&!n(e)}r.d(t,{b:()=>n,Z:()=>a})},44996:(e,t,r)=>{"use strict";r.r(t),r.d(t,{useBaseUrlUtils:()=>i,default:()=>o});var n=r(52263),a=r(13919);function i(){var e=(0,n.default)().siteConfig,t=(e=void 0===e?{}:e).baseUrl,r=void 0===t?"/":t,i=e.url;return{withBaseUrl:function(e,t){return function(e,t,r,n){var i=void 0===n?{}:n,o=i.forcePrependBaseUrl,l=void 0!==o&&o,c=i.absolute,s=void 0!==c&&c;if(!r)return r;if(r.startsWith("#"))return r;if((0,a.b)(r))return r;if(l)return t+r;var u=r.startsWith(t)?r:t+r.replace(/^\//,"");return s?e+u:u}(i,r,e,t)}}}function o(e,t){return void 0===t&&(t={}),(0,i().withBaseUrl)(e,t)}},8802:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e,t){var r=t.trailingSlash,n=t.baseUrl;if(e.startsWith("#"))return e;if(void 0===r)return e;var a,i=e.split(/[#?]/)[0],o="/"===i||i===n?i:(a=i,r?function(e){return e.endsWith("/")?e:e+"/"}(a):function(e){return e.endsWith("/")?e.slice(0,-1):e}(a));return e.replace(i,o)}},18780:function(e,t,r){"use strict";var n=this&&this.__importDefault||function(e){return e&&e.__esModule?e:{default:e}};Object.defineProperty(t,"__esModule",{value:!0}),t.uniq=t.applyTrailingSlash=void 0;var a=r(8802);Object.defineProperty(t,"applyTrailingSlash",{enumerable:!0,get:function(){return n(a).default}});var i=r(29964);Object.defineProperty(t,"uniq",{enumerable:!0,get:function(){return n(i).default}})},29964:(e,t)=>{"use strict";Object.defineProperty(t,"__esModule",{value:!0}),t.default=function(e){return Array.from(new Set(e))}},68629:(e,t,r)=>{"use strict";r.d(t,{Z:()=>p});var n=r(36742),a=r(44256),i=r(67294);function o(){var e=window.encodeURI(JSON.stringify({title:"Feedback about "+window.location.pathname,description:"**!!! Required !!!**\n\nPlease modify the task description to let us know how the docs can be improved.\n\n**Please do not ask support questions via this form! Instead, ask in fburl.com/relay_support**",tag_ids:{add:[0xac96423e5b680,0x64079768ac750]}}));window.open("https://www.internalfb.com/tasks/?n="+e)}function l(e){var t=e.children;return i.createElement("div",{className:"docsRating",id:"docsRating"},i.createElement("hr",null),t)}var c=function(){var e=i.useState(!1),t=e[0],r=e[1],n=function(e){r(!0),function(e){window.ga&&window.ga("send",{hitType:"event",eventCategory:"button",eventAction:"feedback",eventValue:e})}(e)};return t?"Thank you for letting us know!":i.createElement(i.Fragment,null,"Is this page useful?",i.createElement("svg",{className:"i_thumbsup",alt:"Like",id:"docsRating-like",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return n(1)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})),i.createElement("svg",{className:"i_thumbsdown",alt:"Dislike",id:"docsRating-dislike",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 81.13 89.76",onClick:function(){return n(0)}},i.createElement("path",{d:"M22.9 6a18.57 18.57 0 002.67 8.4 25.72 25.72 0 008.65 7.66c3.86 2 8.67 7.13 13.51 11 3.86 3.11 8.57 7.11 11.54 8.45s13.59.26 14.64 1.17c1.88 1.63 1.55 9-.11 15.25-1.61 5.86-5.96 10.55-6.48 16.86-.4 4.83-2.7 4.88-10.93 4.88h-1.35c-3.82 0-8.24 2.93-12.92 3.62a68 68 0 01-9.73.5c-3.57 0-7.86-.08-13.25-.08-3.56 0-4.71-1.83-4.71-4.48h8.42a3.51 3.51 0 000-7H12.28a2.89 2.89 0 01-2.88-2.88 1.91 1.91 0 01.77-1.78h16.46a3.51 3.51 0 000-7H12.29c-3.21 0-4.84-1.83-4.84-4a6.41 6.41 0 011.17-3.78h19.06a3.5 3.5 0 100-7H9.75A3.51 3.51 0 016 42.27a3.45 3.45 0 013.75-3.48h13.11c5.61 0 7.71-3 5.71-5.52-4.43-4.74-10.84-12.62-11-18.71-.15-6.51 2.6-7.83 5.36-8.56m0-6a6.18 6.18 0 00-1.53.2c-6.69 1.77-10 6.65-9.82 14.5.08 5.09 2.99 11.18 8.52 18.09H9.74a9.52 9.52 0 00-6.23 16.9 12.52 12.52 0 00-2.07 6.84 9.64 9.64 0 003.65 7.7 7.85 7.85 0 00-1.7 5.13 8.9 8.9 0 005.3 8.13 6 6 0 00-.26 1.76c0 6.37 4.2 10.48 10.71 10.48h13.25a73.75 73.75 0 0010.6-.56 35.89 35.89 0 007.58-2.18 17.83 17.83 0 014.48-1.34h1.35c4.69 0 7.79 0 10.5-1 3.85-1.44 6-4.59 6.41-9.38.2-2.46 1.42-4.85 2.84-7.62a41.3 41.3 0 003.42-8.13 48 48 0 001.59-10.79c.1-5.13-1-8.48-3.35-10.55-2.16-1.87-4.64-1.87-9.6-1.88a46.86 46.86 0 01-6.64-.29c-1.92-.94-5.72-4-8.51-6.3l-1.58-1.28c-1.6-1.3-3.27-2.79-4.87-4.23-3.33-3-6.47-5.79-9.61-7.45a20.2 20.2 0 01-6.43-5.53 12.44 12.44 0 01-1.72-5.36 6 6 0 00-6-5.86z"})))},s=function(){return i.createElement("p",null,"Let us know how these docs can be improved by",i.createElement("a",{className:"button",role:"button",tabIndex:0,onClick:o},"Filing a task"))},u=function(){return i.createElement("p",null,"Help us make the site even better by"," ",i.createElement(n.default,{to:"https://www.surveymonkey.com/r/FYC9TCJ"},"answering a few quick questions"),".")},d=function(){return i.createElement(l,null,i.createElement(s,null),i.createElement(c,null),i.createElement(u,null))},m=function(){return i.createElement(l,null,i.createElement(c,null),i.createElement(u,null))};const p=function(){return(0,a.fbContent)({internal:i.createElement(d,null),external:i.createElement(m,null)})}},25528:(e,t,r)=>{"use strict";r.r(t),r.d(t,{frontMatter:()=>u,contentTitle:()=>d,metadata:()=>m,toc:()=>p,default:()=>v});var n=r(74034),a=r(79973),i=(r(67294),r(3905)),o=r(44996),l=r(68629),c=r(44256),s=["components"],u={id:"learning-resources",title:"Community Learning Resources",slug:"/community-learning-resources/"},d=void 0,m={unversionedId:"community/learning-resources",id:"version-v11.0.0/community/learning-resources",isDocsHomePage:!1,title:"Community Learning Resources",description:"Relay example projects",source:"@site/versioned_docs/version-v11.0.0/community/learning-resources.md",sourceDirName:"community",slug:"/community-learning-resources/",permalink:"/docs/v11.0.0/community-learning-resources/",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v11.0.0/community/learning-resources.md",tags:[],version:"v11.0.0",lastUpdatedBy:"CodemodService Bot",lastUpdatedAt:1645090648,formattedLastUpdatedAt:"2/17/2022",frontMatter:{id:"learning-resources",title:"Community Learning Resources",slug:"/community-learning-resources/"},sidebar:"version-v11.0.0/docs",previous:{title:"Videos",permalink:"/docs/v11.0.0/principles-and-architecture/videos/"},next:{title:"Glossary",permalink:"/docs/v11.0.0/glossary/"}},p=[{value:"Relay example projects",id:"relay-example-projects",children:[],level:2},{value:"Guides and articles:",id:"guides-and-articles",children:[],level:2},{value:"Relay Modern articles",id:"relay-modern-articles",children:[],level:2}],f={toc:p};function v(e){var t=e.components,r=(0,a.Z)(e,s);return(0,i.mdx)("wrapper",(0,n.Z)({},f,r,{components:t,mdxType:"MDXLayout"}),(0,i.mdx)("h2",{id:"relay-example-projects"},"Relay example projects"),(0,i.mdx)("p",null,"These projects serve as an example of how to use Relay in real world applications."),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://github.com/relayjs/relay-examples"},"github.com/relayjs/relay-examples"))),(0,i.mdx)("h2",{id:"guides-and-articles"},"Guides and articles:"),(0,i.mdx)("ul",null,(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://medium.com/entria/relay-modern-argumentdefinitions-d53769dbb95d"},"How to use @argumentsDefinitions to define local variables to your fragments")," (by Entria)"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://medium.com/entria/wrangling-the-client-store-with-the-relay-modern-updater-function-5c32149a71ac"},"Deep Dive of Updater Relay Store function. How to update your store properly after a mutation or subscription")," (by Entria)"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://medium.com/entria/relay-modern-optimistic-update-a09ba22d83c9"},"Optimistic Update: how to update your UI before server responds")," (by Entria)"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://medium.com/entria/relay-modern-network-deep-dive-ec187629dfd3"},"Relay Network Deep Dive - how to incrementally improve your network layer to manage complex data fetching requirements")," (by Entria)"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://medium.com/@sibelius/relay-modern-migration-to-typescript-c26ab0ee749c"},"Relay Modern with TypeScript - how to configure Relay Modern to make it with TypeScript")," (by @sibelius)"),(0,i.mdx)("li",{parentName:"ul"},(0,i.mdx)("a",{parentName:"li",href:"https://mrtnzlml.com/docs/relay"},"Collection of random thoughts and discoveries around Relay"))),(0,i.mdx)(c.OssOnly,{mdxType:"OssOnly"},(0,i.mdx)("h2",{id:"relay-modern-articles"},"Relay Modern articles"),(0,i.mdx)("p",null,"Note: you can find many more resources by looking at the ",(0,i.mdx)("a",{href:(0,o.default)("docs/v10.1.3/community-learning-resources/")},"Relay Modern Documentation."))),(0,i.mdx)(l.Z,{mdxType:"DocsRating"}))}v.isMDXComponent=!0}}]);
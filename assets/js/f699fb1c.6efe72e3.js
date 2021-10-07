(self.webpackChunk=self.webpackChunk||[]).push([[14606],{3905:(e,t,r)=>{"use strict";r.d(t,{Zo:()=>c,kt:()=>d});var a=r(67294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function p(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},o=Object.keys(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var s=a.createContext({}),l=function(e){var t=a.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},c=function(e){var t=l(e.components);return a.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},h=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,o=e.originalType,s=e.parentName,c=p(e,["components","mdxType","originalType","parentName"]),h=l(r),d=n,m=h["".concat(s,".").concat(d)]||h[d]||u[d]||o;return r?a.createElement(m,i(i({ref:t},c),{},{components:r})):a.createElement(m,i({ref:t},c))}));function d(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=r.length,i=new Array(o);i[0]=h;var p={};for(var s in t)hasOwnProperty.call(t,s)&&(p[s]=t[s]);p.originalType=e,p.mdxType="string"==typeof e?e:n,i[1]=p;for(var l=2;l<o;l++)i[l]=r[l];return a.createElement.apply(null,i)}return a.createElement.apply(null,r)}h.displayName="MDXCreateElement"},76023:(e,t,r)=>{"use strict";r.r(t),r.d(t,{frontMatter:()=>p,contentTitle:()=>s,metadata:()=>l,toc:()=>c,default:()=>h});var a=r(22122),n=r(19756),o=(r(67294),r(3905)),i=["components"],p={id:"prerequisites",title:"Prerequisites",original_id:"prerequisites"},s=void 0,l={unversionedId:"prerequisites",id:"version-v10.0.0/prerequisites",isDocsHomePage:!1,title:"Prerequisites",description:"React",source:"@site/versioned_docs/version-v10.0.0/Introduction-Prerequisites.md",sourceDirName:".",slug:"/prerequisites",permalink:"/docs/v10.0.0/prerequisites",editUrl:"https://github.com/facebook/relay/tree/main/website/versioned_docs/version-v10.0.0/Introduction-Prerequisites.md",version:"v10.0.0",lastUpdatedBy:"Jordan Eldredge",lastUpdatedAt:1633641230,formattedLastUpdatedAt:"10/7/2021",frontMatter:{id:"prerequisites",title:"Prerequisites",original_id:"prerequisites"},sidebar:"version-v10.0.0/docs",previous:{title:"Introduction to Relay",permalink:"/docs/v10.0.0/"},next:{title:"Installation and Setup",permalink:"/docs/v10.0.0/installation-and-setup"}},c=[{value:"React",id:"react",children:[]},{value:"GraphQL",id:"graphql",children:[{value:"A GraphQL Schema",id:"a-graphql-schema",children:[]},{value:"A GraphQL Server",id:"a-graphql-server",children:[]}]}],u={toc:c};function h(e){var t=e.components,r=(0,n.Z)(e,i);return(0,o.kt)("wrapper",(0,a.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h2",{id:"react"},"React"),(0,o.kt)("p",null,"Relay is a framework for data management with the primary supported binding for React applications, so we assume that you are already familiar with ",(0,o.kt)("a",{parentName:"p",href:"https://reactjs.org/"},"React"),"."),(0,o.kt)("h2",{id:"graphql"},"GraphQL"),(0,o.kt)("p",null,"We also assume basic understanding of ",(0,o.kt)("a",{parentName:"p",href:"http://graphql.org/learn/"},"GraphQL"),". In order to start using Relay, you will also need:"),(0,o.kt)("h3",{id:"a-graphql-schema"},"A GraphQL Schema"),(0,o.kt)("p",null,"A description of your data model with an associated set of resolve methods that know how to fetch any data your application could ever need."),(0,o.kt)("p",null,"GraphQL is designed to support a wide range of data access patterns. In order to understand the structure of an application's data, Relay requires that you follow certain conventions when defining your schema. These are documented in the ",(0,o.kt)("a",{parentName:"p",href:"./graphql-server-specification"},"GraphQL Server Specification"),"."),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},(0,o.kt)("strong",{parentName:"p"},(0,o.kt)("a",{parentName:"strong",href:"https://github.com/graphql/graphql-js"},"graphql-js"))," on ",(0,o.kt)("a",{parentName:"p",href:"https://www.npmjs.com/package/graphql"},"npm")),(0,o.kt)("p",{parentName:"li"},"General-purpose tools for building a GraphQL schema using JavaScript")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("p",{parentName:"li"},(0,o.kt)("strong",{parentName:"p"},(0,o.kt)("a",{parentName:"strong",href:"https://github.com/graphql/graphql-relay-js"},"graphql-relay-js"))," on ",(0,o.kt)("a",{parentName:"p",href:"https://www.npmjs.com/package/graphql-relay"},"npm")),(0,o.kt)("p",{parentName:"li"},"JavaScript helpers for defining connections between data, and mutations, in a way that smoothly integrates with Relay."))),(0,o.kt)("h3",{id:"a-graphql-server"},"A GraphQL Server"),(0,o.kt)("p",null,"Any server can be taught to load a schema and speak GraphQL. Our ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/relayjs/relay-examples"},"examples")," use Express."),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},(0,o.kt)("a",{parentName:"strong",href:"https://github.com/graphql/express-graphql"},"express-graphql"))," on ",(0,o.kt)("a",{parentName:"li",href:"https://www.npmjs.com/package/express-graphql"},"npm")),(0,o.kt)("li",{parentName:"ul"},(0,o.kt)("strong",{parentName:"li"},(0,o.kt)("a",{parentName:"strong",href:"https://www.graph.cool/"},"Graphcool"))," (",(0,o.kt)("a",{parentName:"li",href:"https://www.graph.cool/docs/quickstart/"},"Quickstart tutorial"),")")))}h.isMDXComponent=!0}}]);
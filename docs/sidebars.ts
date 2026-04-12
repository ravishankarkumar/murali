import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'coordinate-system',
    {
      type: 'category',
      label: 'Tattvas',
      link: {type: 'doc', id: 'tattvas/index'},
      items: [
        'tattvas/primitives',
        'tattvas/text',
        'tattvas/composite',
        'tattvas/graphs',
        'tattvas/math',
        'tattvas/layout',
        'tattvas/storytelling',
        'tattvas/utility',
      ],
    },
    'animations',
    'scene-and-app',
    'camera',
    'updaters',
    'roadmap',
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'architecture/overview',
        'architecture/architecture',
        'architecture/scene-timeline',
        'architecture/tattva',
        'architecture/dirty-flags',
        'architecture/projection',
        'architecture/ecs',
        'architecture/renderer',
        'architecture/text-and-latex',
        'architecture/end-to-end-flow',
      ],
    },
    {
      type: 'category',
      label: 'Feature Internals',
      items: [
        'feature-internals/overview',
        'feature-internals/stepwise',
        'feature-internals/neural-network',
      ],
    },

  ],
};

export default sidebars;

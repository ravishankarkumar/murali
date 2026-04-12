import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link className="button button--secondary button--lg" to="/docs/intro">
            Get started
          </Link>
          <Link
            className="button button--outline button--secondary button--lg"
            href="https://github.com/ravishankarkumar/murali"
            style={{marginLeft: '1rem'}}>
            GitHub
          </Link>
        </div>
      </div>
    </header>
  );
}

const features = [
  {
    title: 'GPU-accelerated',
    description: 'Built on wgpu — runs on Vulkan, Metal, and DX12. No OpenGL, no deprecation surprises.',
  },
  {
    title: 'Time-driven',
    description: 'Animations are pure functions of time. Same input always produces the same output.',
  },
  {
    title: 'LaTeX & Typst',
    description: 'Embedded math typesetting. Write LaTeX expressions and they render as crisp textures in world space.',
  },
  {
    title: 'Strongly typed',
    description: 'Rust\'s type system catches layout and animation errors at compile time, not at render time.',
  },
];

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout title={siteConfig.title} description={siteConfig.tagline}>
      <HomepageHeader />
      <main>
        <section style={{padding: '3rem 0'}}>
          <div className="container">
            <div className="row">
              {features.map(({title, description}) => (
                <div key={title} className="col col--3">
                  <div style={{padding: '1rem'}}>
                    <Heading as="h3">{title}</Heading>
                    <p>{description}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
